use anchor_lang::prelude::*;
use crate::state::ProgramState;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;
use crate::errors::ErrorCode::AlreadyInitialized;
 pub fn initialize(ctx : Context<InitializedCtx>) -> Result<()> {

    let state =  &mut ctx.accounts.program_state;

    let deployer = &ctx.accounts.deployer;


    if state.initialized {
        return Err(AlreadyInitialized.into());
    }


    state.campaign_count = 0;
    state.platform_fee = 5;
    state.platform_address = deployer.key();
    state.initialized = true;


    Ok(())
 }



 #[derive(Accounts)]

 pub struct InitializedCtx <'info> {

    #[account(
        init,
        payer = deployer,
        space = ANCHOR_DISCRIMINATOR_SIZE + ProgramState::INIT_SPACE,
        seeds = [b"program_state"],
        bump
    )]

    pub program_state : Account<'info,ProgramState>,
    

 #[account(mut)]
deployer : Signer<'info>,


pub system_program : Program<'info, System>
 
 
}