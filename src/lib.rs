use std::u64;

use borsh::{BorshDeserialize, BorshSerialize};
use solana_program ::{
account_info::{AccountInfo, next_account_info},entrypoint,entrypoint::{ ProgramResult}, msg, program::{invoke, invoke_signed}, program_error::ProgramError, pubkey::Pubkey, rent::{self, Rent}, system_instruction, sysvar::Sysvar
};

#[derive(BorshDeserialize,BorshSerialize)]
struct EscrowState {
maker:Pubkey,
taker:Pubkey,
amount:u64,
is_released:bool

}


#[derive(BorshDeserialize,BorshSerialize)]
enum EscrowInstruction {
    InitEscrow { amount:u64 },
    Release ,
    Cancel,
}

entrypoint!(process_instruction);

pub fn process_instruction (
    program_id:&Pubkey,
    accounts:&[AccountInfo],
    instruction_data : &[u8],

)    ->ProgramResult {

msg!(" Escrow program entrypoint ");

let instruction = EscrowInstruction::try_from_slice(instruction_data)?;
let accounts_iter = &mut accounts.iter();
let maker_acc = next_account_info(accounts_iter)?;
let taker_acc = next_account_info(accounts_iter)?;
let escrow_acc =next_account_info(accounts_iter)?;
let sys_program=next_account_info(accounts_iter)?;


let pda_address = Pubkey::find_program_address(&[b"escrow",maker_acc.key.as_ref(),taker_acc.key.as_ref()], program_id);

if *escrow_acc.key != pda_address.0 {
     
     return  Err(ProgramError::InvalidAccountOwner);

}

match instruction {

    EscrowInstruction::InitEscrow { amount } => {
      let escrow_state = EscrowState {
        maker : *maker_acc.key,
        taker:*taker_acc.key,
        amount:amount,
        is_released:false,  
      };
      let rent = Rent::get()?;
      let required_lamports = rent.minimum_balance(std::mem::size_of::<EscrowState>());

    if !maker_acc.is_signer {
      return Err(ProgramError::MissingRequiredSignature);
    }
    let instructions = system_instruction::create_account(maker_acc.key, &*escrow_acc.key, required_lamports, std::mem::size_of::<EscrowState>() as u64, program_id);

    invoke_signed(&instructions, &[maker_acc.clone(), escrow_acc.clone(), sys_program.clone()], &[&[b"escrow",maker_acc.key.as_ref(),taker_acc.key.as_ref(),&[pda_address.1]]])?;

    let trasnfer_ins = system_instruction::transfer(maker_acc.key, escrow_acc.key, amount);

    invoke(&trasnfer_ins, &[maker_acc.clone(),escrow_acc.clone()])?;

        escrow_state.serialize(&mut *escrow_acc.data.borrow_mut())?;
       msg!("Escrow initialized!");

    }

    EscrowInstruction::Release => {

     let mut state = EscrowState::try_from_slice(&escrow_acc.data.borrow())?;

     if maker_acc.key != &state.maker {
       return Err(ProgramError::InvalidArgument);
     }

     if !maker_acc.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
     }
     
    if state.is_released {
        return Err(ProgramError::InvalidArgument);
    }

    state.is_released=true;
    **escrow_acc.try_borrow_mut_lamports()? -= state.amount;
    **taker_acc.try_borrow_mut_lamports()? += state.amount;

    state.serialize(&mut *escrow_acc.data.borrow_mut())?;
    msg!("Escrow released!");

    }

    EscrowInstruction::Cancel => {

         let state = EscrowState::try_from_slice(&escrow_acc.data.borrow())?;


     if maker_acc.key != &state.maker {
       return Err(ProgramError::InvalidArgument);
     }

     if !maker_acc.is_signer {
        return Err(ProgramError::MissingRequiredSignature);
     }
     
    if state.is_released {
        return Err(ProgramError::InvalidArgument);
    }

    **escrow_acc.try_borrow_mut_lamports()? -= state.amount;
**maker_acc.try_borrow_mut_lamports()? += state.amount;

    msg!("Escrow Canceled!");
         
    }
}

Ok(())

}
