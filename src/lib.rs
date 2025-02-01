use borsh::{BorshDeserialize, BorshSerialize};
use solana_program::{
    account_info::{next_account_info, AccountInfo},
    entrypoint,
    entrypoint::ProgramResult,
    msg,
    pubkey::Pubkey,
};

pub mod instruction;
use crate::instruction::CounterInstructions;

#[derive(Debug, BorshDeserialize, BorshSerialize)]
pub struct CounterAccount {
    pub counter: u32,
}

entrypoint!(process_instructions);

pub fn process_instructions(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    msg!("Counter program entrypoint");

    let instruction = CounterInstructions::unpack(instruction_data)?;

    let accounts_iter = &mut accounts.iter();
    let account = next_account_info(accounts_iter)?;

    let mut counter_account = CounterAccount::try_from_slice(&account.data.borrow())?;

    match instruction {
        CounterInstructions::Increment(v) => counter_account.counter += v.value,
        CounterInstructions::Decrement(v) => counter_account.counter -= v.value,
        CounterInstructions::Reset => counter_account.counter = 0,
        CounterInstructions::Update(v) => counter_account.counter = v.value,
    };

    counter_account.serialize(&mut &mut account.data.borrow_mut()[..])?;
    return Ok(());
}

#[cfg(test)]
mod test {
    use super::*;
    use solana_program::{clock::Epoch, pubkey::Pubkey};
    use std::mem;

    #[test]
    fn test_counter() {
        let program_id = Pubkey::default();
        let key = Pubkey::default();
        let mut lamports = 0;
        let mut data = vec![0; mem::size_of::<u32>()];
        let owner = Pubkey::default();

        let account = AccountInfo::new(
            &key,
            false,
            true,
            &mut lamports,
            &mut data,
            &owner,
            false,
            Epoch::default(),
        );
        let accounts = vec![account];

        let mut increment_instruction_data = vec![0u8];
        let mut decrement_instruction_data = vec![1u8];
        let mut update_instruction_data = vec![2u8];
        let reset_instruction_data = vec![3u8];

        // Increment
        increment_instruction_data.extend_from_slice(&2u32.to_le_bytes());
        process_instructions(&program_id, &accounts, &increment_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            2
        );

        // Decrement
        decrement_instruction_data.extend_from_slice(&1u32.to_le_bytes());
        process_instructions(&program_id, &accounts, &decrement_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            1
        );

        // Update
        update_instruction_data.extend_from_slice(&65u32.to_le_bytes());
        process_instructions(&program_id, &accounts, &update_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            65
        );

        // Reset
        process_instructions(&program_id, &accounts, &reset_instruction_data).unwrap();
        assert_eq!(
            CounterAccount::try_from_slice(&accounts[0].data.borrow())
                .unwrap()
                .counter,
            0
        );
    }
}
