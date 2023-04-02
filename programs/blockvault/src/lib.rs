use anchor_lang::prelude::*;

declare_id!("NO!");

#[program]
pub mod blockvault {
    use super::*;

    pub fn upload_file(ctx: Context<UploadFile>, title: String, description: String, cid: String) -> Result<()> {
        let file: &mut Account<File> = &mut ctx.accounts.file;
        let owner: &Signer = &ctx.accounts.owner;
        let clock: Clock = Clock::get().unwrap();

        if title.chars().count() > 50 {
            return Err(ErrorCode::TitleTooLong.into());
        } if description.chars().count() > 300 {
            return Err(ErrorCode::DescriptionTooLong.into());
        }

        file.owner = *owner.key;
        file.title = title;
        file.timestamp = clock.unix_timestamp;
        file.description = description;
        file.filecid = cid;
        Ok(())
    }
}

#[derive(Accounts)]
pub struct UploadFile<'info> {
    #[account(init, payer=owner, space=File::LEN)]
    pub file: Account<'info, File>,

    #[account(mut)]
    pub owner: Signer<'info>,

    pub system_program: Program<'info, System>,
}


#[account]
pub struct File {
    pub owner: Pubkey,
    pub timestamp: i64,
    pub title: String,
    pub description: String,
    pub filecid: String
}

const DISCRIMINATOR_LENGTH: usize = 8;
const PUBLIC_KEY_LENGTH: usize = 32;
const TIMESTAMP_LENGTH: usize = 8;
const STRING_LENGTH_PREFIX: usize = 4;
const MAX_TITLE_LENGTH: usize = 4 * 50;
const MAX_DESCRIPTION_LENGTH: usize = 4 * 300;
const FILE_CID_LENGTH: usize = 256;

impl File {
    const LEN: usize = DISCRIMINATOR_LENGTH                     // Identifies Account
                + PUBLIC_KEY_LENGTH                             // File Owner
                + TIMESTAMP_LENGTH                              // Time at which Uploaded
                + STRING_LENGTH_PREFIX + MAX_TITLE_LENGTH       // File Title
                + STRING_LENGTH_PREFIX + MAX_DESCRIPTION_LENGTH // File Description
                + STRING_LENGTH_PREFIX + FILE_CID_LENGTH;       // File CID at IPFS
}

#[error_code]
pub enum ErrorCode {
    #[msg("Topic Exceeds 50 Characters.")]
    TitleTooLong,

    #[msg("Content Exceeds 280 Characters.")]
    DescriptionTooLong
}
