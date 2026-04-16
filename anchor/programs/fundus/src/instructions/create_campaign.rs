use anchor_lang::prelude::*;

use crate::state::ProgramState;
use crate::state::Campaign;
use crate::errors::ErrorCode;
use crate::constants::ANCHOR_DISCRIMINATOR_SIZE;

pub fn create_campaign(ctx : Context<CreateCampaignCtx>,
    title : String,
     description : String, 
     image_url : String, 
     goal: u64

) -> Result<()>{
    let campaign =  &mut ctx.accounts.campaign;
    let state = &mut ctx.accounts.program_state;



    require!(title.len() <= 64, ErrorCode::TitleTooLong);
    require!(description.len() <= 512, ErrorCode::DescriptionTooLong);
    require!(image_url.len() <= 256, ErrorCode::ImageUrlTooLong);

    if goal < 1_000_000_000 {
        return Err(ErrorCode::InvalidGoalAmount.into())
    }
    //require! macro throws if there is a false statement error is thrown


    state.campaign_count +=1;
    campaign.creator = ctx.accounts.creator.key();

    campaign.title = title;
    campaign.description = description;
    campaign.image_url = image_url;
    campaign.goal = goal;

    campaign.amount_raised = 0;
    campaign.donors = 0;
    campaign.withdrawls = goal;
    campaign.timestamp = Clock::get()?.unix_timestamp as u64;
    campaign.active = true;
    Ok(())
}



#[derive(Accounts)]

pub struct  CreateCampaignCtx<'info>{
    pub program_state : Account<'info, ProgramState>,

    #[account(
        init,
        payer = creator,
        space = ANCHOR_DISCRIMINATOR_SIZE + Campaign::INIT_SPACE,
        seeds = [b"campaign", (program_state.campaign_count + 1 ).to_le_bytes().as_ref()

        ],
        bump
        
    )]
    pub campaign :Account<'info , Campaign>,


    #[account(mut)]
    pub creator : Signer<'info>,

   pub  system_program : Program<'info, System>
}