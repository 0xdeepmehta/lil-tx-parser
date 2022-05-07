use anchor_lang::{
    prelude::*,
    solana_program::{
        sysvar,
        sysvar::{instructions::get_instruction_relative}
    },
};
declare_id!("8PL5wn76xtHShBxYCCkVpdzVMqf8KrUdk64aS3Sscwnx");

// pre-computed sighash of Ixs
const PREV_IX_OPCODE: u64 = 0x9a078452f4962bdf;
const POST_IX_OPCODE: u64 = 0xf3cc1f496019f42f;


#[program]
pub mod lil_tx_parser {
    use super::*;

    pub fn cur_ix(ctx: Context<CurrentIx>) -> Result<()> {

        let instruction_sysvar_account = ctx.accounts.instruction_sysvar_account.to_account_info();

        // Current IX
        let current_ix = get_instruction_relative(0, &instruction_sysvar_account).unwrap();
        msg!("Account it contains :: {}", current_ix.accounts[0].pubkey);

        // Pre Ix
        let pre_ix = get_instruction_relative(-1, &instruction_sysvar_account).unwrap();
        if pre_ix.program_id == crate::ID && u64::from_be_bytes(pre_ix.data[..8].try_into().unwrap()) == PREV_IX_OPCODE {
            msg!("Prev ix seems prefactoooo");
            msg!("Account it contains :: {}", pre_ix.accounts[0].pubkey);
        } else {
            return Err(ParserError::InvalidPreIx.into());
        }

        // Post Ix
        let post_ix = get_instruction_relative(1, &instruction_sysvar_account).unwrap();
        if post_ix.program_id == crate::ID && u64::from_be_bytes(post_ix.data[..8].try_into().unwrap()) == POST_IX_OPCODE {
            msg!("aah! Post ix also seems prefactoooo");
            msg!("Account it contains :: {}", post_ix.accounts[0].pubkey);
        } else { return Err(ParserError::InvalidPostIx.into()) }
        Ok(())
    }

    pub fn pre_ix(_ctx: Context<PrevIx>) -> Result<()> {
        msg!("Someone executed me lol");
        Ok(())
    }

    pub fn post_ix(_ctx: Context<PostIx>) -> Result<()> {
        msg!("aah me last!");
        Ok(())
    }

}

#[derive(Accounts)]
pub struct CurrentIx<'info> {
    /// CHECK: Just for DEMO
    pub curnt_account_id: AccountInfo<'info>,

    /// CHECK: instruction_sysvar_account cross checking 
    #[account(address = sysvar::instructions::ID)]
    instruction_sysvar_account: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PrevIx<'info> {
    /// CHECK: Just for DEMO
    pub pre_account_id: AccountInfo<'info>,
}

#[derive(Accounts)]
pub struct PostIx<'info> {
    /// CHECK: Just for DEMO
    pub post_account_id: AccountInfo<'info>,
}

#[error_code]
pub enum ParserError {
    #[msg("Invalid Pre Ix")]
    InvalidPreIx,

    #[msg("Invalid Post Ix")]
    InvalidPostIx,
}

