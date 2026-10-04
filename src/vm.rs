use crate::{instruction, state};

pub enum VMError {
    ProgramNotProvided,
    InstructionOutOfBound,
    StackMissingExpectedValue,
    StackInvalidExpectedValueKind,
    ArithmeticValueOverflow,
    StateMissingReference,
    StateTransferNotEnoughBalance,
    StateTransferBalanceOverflow,
    StateTransferInvalidReceiver,
    StateAccountNonceOverflow,
}

enum VMArithmeticOperation {
    Add,
    Sub,
    Mul,
    MulPercentage,
}

pub struct VM {
    pc: usize,
    stack: Vec<instruction::Value>,
    program: instruction::Program,
    state: state::State,
}

impl VM {
    /// runs every instruction given as program and returns the modify state based on initial one.
    /// May Error on specific instructions but working state is always guaranted to be semanticly
    /// correct
    /// no longer use after firt execution
    pub fn run(mut self) -> Result<state::State, VMError> {
        loop {
            let i = self
                .program
                .get(self.pc)
                .ok_or(VMError::InstructionOutOfBound)?;

            match i {
                instruction::Instruction::Add => self.handle_arithmetic(VMArithmeticOperation::Add),
                instruction::Instruction::Sub => self.handle_arithmetic(VMArithmeticOperation::Sub),
                instruction::Instruction::Mul => self.handle_arithmetic(VMArithmeticOperation::Mul),
                instruction::Instruction::MulPercentage => {
                    self.handle_arithmetic(VMArithmeticOperation::MulPercentage)
                }

                instruction::Instruction::Load => self.handle_load(),
                instruction::Instruction::Transfer => self.handle_transfer(),

                instruction::Instruction::Push(val) => {
                    self.stack.push(val.clone());
                    Ok(())
                }
                instruction::Instruction::Halt => break,
            }?;

            self.pc += 1;
        }

        Ok(self.state)
    }

    pub fn program(&self) -> &instruction::Program {
        &self.program
    }

    pub fn builder() -> VMBuilder {
        VMBuilder::default()
    }

    fn handle_arithmetic(&mut self, arit: VMArithmeticOperation) -> Result<(), VMError> {
        let el2 = self.extract_amount()?;
        let el1 = self.extract_amount()?;

        let res = match arit {
            VMArithmeticOperation::Add => {
                el1.checked_add(el2).ok_or(VMError::ArithmeticValueOverflow)
            }
            VMArithmeticOperation::Sub => {
                el1.checked_sub(el2).ok_or(VMError::ArithmeticValueOverflow)
            }

            VMArithmeticOperation::Mul => {
                el1.checked_mul(el2).ok_or(VMError::ArithmeticValueOverflow)
            }

            VMArithmeticOperation::MulPercentage => el1
                .checked_mul(el2)
                .ok_or(VMError::ArithmeticValueOverflow)?
                .checked_div(100)
                .ok_or(VMError::ArithmeticValueOverflow),
        }?;

        self.stack.push(instruction::Value::Amount(res));
        Ok(())
    }

    fn handle_load(&mut self) -> Result<(), VMError> {
        let code = self.extract_code()?;
        let acc = self
            .state
            .account(&code)
            .ok_or(VMError::StateMissingReference)?;

        self.stack.push(instruction::Value::Amount(acc.bal));
        Ok(())
    }

    fn handle_transfer(&mut self) -> Result<(), VMError> {
        let rec_code = self.extract_code()?;
        let amount = self.extract_amount()?;

        let new_send = if let Some(send_code) = self.state.owner_code().map(str::to_owned) {
            if send_code == rec_code {
                return Err(VMError::StateTransferInvalidReceiver);
            }

            let send = self
                .state
                .account(&send_code)
                .ok_or(VMError::StateMissingReference)?;

            Some((
                send_code,
                send.bal
                    .checked_sub(amount)
                    .ok_or(VMError::StateTransferNotEnoughBalance)?,
                send.nonce
                    .checked_add(1)
                    .ok_or(VMError::StateAccountNonceOverflow)?,
            ))
        } else {
            None
        };

        let rec = self
            .state
            .account(&rec_code)
            .ok_or(VMError::StateMissingReference)?;

        let new_rec_bal = rec
            .bal
            .checked_add(amount)
            .ok_or(VMError::StateTransferBalanceOverflow)?;

        if let Some((send_code, send_bal, send_nonce)) = new_send {
            let send = self.state.account_mut(&send_code).unwrap();
            send.bal = send_bal;
            send.nonce = send_nonce;
        }

        let rec = self.state.account_mut(&rec_code).unwrap();
        rec.bal = new_rec_bal;

        Ok(())
    }

    // STACK MANIPULATION RELATED

    fn extract_amount(&mut self) -> Result<u128, VMError> {
        match self.stack.pop().ok_or(VMError::StackMissingExpectedValue)? {
            instruction::Value::Amount(val) => Ok(val),
            _ => Err(VMError::StackInvalidExpectedValueKind),
        }
    }

    fn extract_code(&mut self) -> Result<String, VMError> {
        match self.stack.pop().ok_or(VMError::StackMissingExpectedValue)? {
            instruction::Value::Code(val) => Ok(val),
            _ => Err(VMError::StackInvalidExpectedValueKind),
        }
    }
}

#[derive(Default)]
pub struct VMBuilder {
    program: Option<instruction::Program>,
    state: Option<state::State>,
}

impl VMBuilder {
    pub fn program(mut self, program: instruction::Program) -> Self {
        self.program = Some(program);
        self
    }

    pub fn state(mut self, state: state::State) -> Self {
        self.state = Some(state);
        self
    }

    pub fn build(self) -> Result<VM, VMError> {
        let program = self.program.ok_or(VMError::ProgramNotProvided)?;
        let state = self.state.unwrap_or_default();
        let pc = 0;
        let stack = Vec::new();
        Ok(VM {
            program,
            state,
            pc,
            stack,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    use instruction::{Instruction, Program, Value};
    use state::{Account, State};

    fn state(owner: Option<&str>, accounts: &[(&str, u128, u64)]) -> State {
        let accounts = accounts
            .iter()
            .map(|(code, bal, nonce)| {
                (
                    (*code).to_owned(),
                    Account {
                        bal: *bal,
                        nonce: *nonce,
                    },
                )
            })
            .collect::<HashMap<_, _>>();

        let builder = State::builder().accounts(accounts);
        match owner {
            Some(code) => builder.owner_code(code.to_owned()).build(),
            None => builder.build(),
        }
    }

    fn vm_with(stack: Vec<Value>, state: State) -> VM {
        VM {
            pc: 0,
            stack,
            program: Program::with_instructions(vec![Instruction::Halt]).unwrap(),
            state,
        }
    }

    #[test]
    fn builder_requires_a_program() {
        assert!(matches!(
            VM::builder().build(),
            Err(VMError::ProgramNotProvided)
        ));
    }

    #[test]
    fn arithmetic_handlers_apply_checked_operations_in_stack_order() {
        let cases = [
            (Instruction::Add, 10, 4, 14),
            (Instruction::Sub, 10, 4, 6),
            (Instruction::Mul, 10, 4, 40),
            (Instruction::MulPercentage, 250, 20, 50),
        ];

        for (instruction, lhs, rhs, expected) in cases {
            let mut vm = vm_with(
                vec![Value::Amount(lhs), Value::Amount(rhs)],
                State::default(),
            );
            let operation = match instruction {
                Instruction::Add => VMArithmeticOperation::Add,
                Instruction::Sub => VMArithmeticOperation::Sub,
                Instruction::Mul => VMArithmeticOperation::Mul,
                Instruction::MulPercentage => VMArithmeticOperation::MulPercentage,
                _ => unreachable!(),
            };

            assert!(matches!(vm.handle_arithmetic(operation), Ok(())));

            assert_eq!(vm.stack, vec![Value::Amount(expected)]);
        }
    }

    #[test]
    fn arithmetic_handlers_report_missing_or_wrongly_typed_operands() {
        let mut empty = vm_with(vec![], State::default());
        assert!(matches!(
            empty.handle_arithmetic(VMArithmeticOperation::Add),
            Err(VMError::StackMissingExpectedValue)
        ));

        let mut wrong_kind = vm_with(
            vec![Value::Amount(1), Value::Code("not-an-amount".to_owned())],
            State::default(),
        );
        assert!(matches!(
            wrong_kind.handle_arithmetic(VMArithmeticOperation::Add),
            Err(VMError::StackInvalidExpectedValueKind)
        ));
    }

    #[test]
    fn arithmetic_handlers_report_checked_arithmetic_failures() {
        let cases = [
            (VMArithmeticOperation::Add, u128::MAX, 1),
            (VMArithmeticOperation::Sub, 1, 2),
            (VMArithmeticOperation::Mul, u128::MAX, 2),
            (VMArithmeticOperation::MulPercentage, u128::MAX, 2),
        ];

        for (operation, lhs, rhs) in cases {
            let mut vm = vm_with(
                vec![Value::Amount(lhs), Value::Amount(rhs)],
                State::default(),
            );
            assert!(matches!(
                vm.handle_arithmetic(operation),
                Err(VMError::ArithmeticValueOverflow)
            ));
        }
    }

    #[test]
    fn load_pushes_the_referenced_account_balance() {
        let mut vm = vm_with(
            vec![Value::Code("account".to_owned())],
            state(None, &[("account", 42, 7)]),
        );

        assert!(matches!(vm.handle_load(), Ok(())));

        assert_eq!(vm.stack, vec![Value::Amount(42)]);
    }

    #[test]
    fn load_reports_missing_stack_values_wrong_kinds_and_missing_accounts() {
        let mut empty = vm_with(vec![], State::default());
        assert!(matches!(
            empty.handle_load(),
            Err(VMError::StackMissingExpectedValue)
        ));

        let mut wrong_kind = vm_with(vec![Value::Amount(1)], State::default());
        assert!(matches!(
            wrong_kind.handle_load(),
            Err(VMError::StackInvalidExpectedValueKind)
        ));

        let mut missing_account = vm_with(vec![Value::Code("absent".to_owned())], State::default());
        assert!(matches!(
            missing_account.handle_load(),
            Err(VMError::StateMissingReference)
        ));
    }

    #[test]
    fn transfer_debits_owner_credits_receiver_and_increments_owner_nonce() {
        let mut vm = vm_with(
            vec![Value::Amount(12), Value::Code("receiver".to_owned())],
            state(Some("owner"), &[("owner", 50, 3), ("receiver", 8, 9)]),
        );

        assert!(matches!(vm.handle_transfer(), Ok(())));

        let owner = vm.state.account("owner").unwrap();
        let receiver = vm.state.account("receiver").unwrap();
        assert_eq!((owner.bal, owner.nonce), (38, 4));
        assert_eq!((receiver.bal, receiver.nonce), (20, 9));
    }

    #[test]
    fn transfer_without_owner_credits_receiver_without_debit_or_nonce_change() {
        let mut vm = vm_with(
            vec![Value::Amount(12), Value::Code("receiver".to_owned())],
            state(None, &[("receiver", 8, 9)]),
        );

        assert!(matches!(vm.handle_transfer(), Ok(())));

        let receiver = vm.state.account("receiver").unwrap();
        assert_eq!((receiver.bal, receiver.nonce), (20, 9));
    }

    #[test]
    fn failed_transfers_leave_account_balances_and_nonces_unchanged() {
        let cases = [
            (
                state(Some("owner"), &[("owner", 5, 2), ("receiver", 7, 4)]),
                6,
                "receiver",
                VMError::StateTransferNotEnoughBalance,
            ),
            (
                state(
                    Some("owner"),
                    &[("owner", 5, 2), ("receiver", u128::MAX, 4)],
                ),
                1,
                "receiver",
                VMError::StateTransferBalanceOverflow,
            ),
            (
                state(Some("owner"), &[("owner", 5, u64::MAX), ("receiver", 7, 4)]),
                1,
                "receiver",
                VMError::StateAccountNonceOverflow,
            ),
        ];

        for (state, amount, receiver_code, expected_error) in cases {
            let mut vm = vm_with(
                vec![Value::Amount(amount), Value::Code(receiver_code.to_owned())],
                state,
            );
            let before = [
                vm.state
                    .account("owner")
                    .map(|account| (account.bal, account.nonce)),
                vm.state
                    .account("receiver")
                    .map(|account| (account.bal, account.nonce)),
            ];

            let result = vm.handle_transfer();

            match expected_error {
                VMError::StateTransferNotEnoughBalance => assert!(matches!(
                    result,
                    Err(VMError::StateTransferNotEnoughBalance)
                )),
                VMError::StateTransferBalanceOverflow => {
                    assert!(matches!(result, Err(VMError::StateTransferBalanceOverflow)))
                }
                VMError::StateAccountNonceOverflow => {
                    assert!(matches!(result, Err(VMError::StateAccountNonceOverflow)))
                }
                _ => unreachable!(),
            }

            let after = [
                vm.state
                    .account("owner")
                    .map(|account| (account.bal, account.nonce)),
                vm.state
                    .account("receiver")
                    .map(|account| (account.bal, account.nonce)),
            ];
            assert_eq!(after, before);
        }
    }

    #[test]
    fn transfer_rejects_owner_as_receiver_without_changing_state() {
        let mut vm = vm_with(
            vec![Value::Amount(1), Value::Code("owner".to_owned())],
            state(Some("owner"), &[("owner", 10, 2)]),
        );

        assert!(matches!(
            vm.handle_transfer(),
            Err(VMError::StateTransferInvalidReceiver)
        ));
        let owner = vm.state.account("owner").unwrap();
        assert_eq!((owner.bal, owner.nonce), (10, 2));
    }

    #[test]
    fn transfer_reports_missing_owner_or_receiver_without_partial_debit() {
        let mut missing_owner = vm_with(
            vec![Value::Amount(1), Value::Code("receiver".to_owned())],
            state(Some("absent-owner"), &[("receiver", 10, 2)]),
        );
        assert!(matches!(
            missing_owner.handle_transfer(),
            Err(VMError::StateMissingReference)
        ));
        assert_eq!(
            (
                missing_owner.state.account("receiver").unwrap().bal,
                missing_owner.state.account("receiver").unwrap().nonce
            ),
            (10, 2)
        );

        let mut missing_receiver = vm_with(
            vec![Value::Amount(1), Value::Code("absent-receiver".to_owned())],
            state(Some("owner"), &[("owner", 10, 2)]),
        );
        assert!(matches!(
            missing_receiver.handle_transfer(),
            Err(VMError::StateMissingReference)
        ));
        let owner = missing_receiver.state.account("owner").unwrap();
        assert_eq!((owner.bal, owner.nonce), (10, 2));
    }
}
