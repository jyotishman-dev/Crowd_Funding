use anchor_lang::prelude::*;

#[error_code]
pub enum ErrorCode {
    #[msg("The Program has already be initialized")]
    AlreadyInitialized,

    #[msg("Title exceeds the maximum length of 64 char")]
    TitleTooLong,

    #[msg("Description Exceeds the maximum of 512 char")]
    DescriptionTooLong,

    #[msg("Image url exceeds the maximum length of 256 characters")]
    ImageUrlTooLong,

    #[msg("Invalid goal amount . Goal must be greater than zero")]
    InvalidGoalAmount,

    #[msg("You do not have the access to this campaign")]
    Unauthorized,


    #[msg("Campaign not found for the cid")]
    CampaignNotFound,

     #[msg("Campaign is already deleted or not present")]
    InactiveCampaign,
}