#![no_std]

use core::f32::consts::PI;

use pinocchio::{account_info::AccountInfo, entrypoint, program_error::ProgramError, pubkey::Pubkey, ProgramResult}

entrypoint!(process_instruction);

fn process_instruction(
    _program_id: &Pubkey,
    accounts: &[AccountInfo],
    instruction_data: &[u8],
) -> ProgramResult {
    match instruction_data.split_first() {
        Some((Deposit::DISCRIMINATOR, data)) => Deposit::try_from((data, accounts))?.process(),
        Some((Withdraw::DISCRIMINATOR, _)) => Withdraw::try_from(accounts)?.process(),
        _ => Err(ProgramError::InvalidInstructionData)
    }
}

pub struct DepositAccounts<'a> {
    pub owner: &'a AccountInfo,
    pub vault: &'a AccountInfo,
}

 impl<'a> TryFrom<&'a [AccountInfo]> for DepositAccounts<'a> {
   type Error = ProgramError;

   fn try_from(accounts: &'a [AccountInfo]) -> Result<Self, Self::Error> {
       let [owner, vault, _] = accounts else {
        return Err(ProgramError::NotEnoughAccountKeys);
       };

       if !owner.is_signer() {
          return Err(ProgramError::InvalidInstructionOwner);
       }

       if vault.owner().ne(&pinocchio_system::ID) {
          return Err(ProgramError::InvalidInstructionOwner);
       }

       if vault.lamports().ne(&0) {
          return Err(ProgramError::InvalidAccountData);
       }

       let (vault_key, _) = find_program_address(&[b"vault", owner.key()], &crate::ID);
       if vault.key().ne(&vault_key) {
          return Err(ProgramError::InvalidAccountOwner)
       }
     
         Ok(Self { owner, vault})
   }
}