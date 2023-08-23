use crate::{pda_seeds, state::Pda};
use anchor_lang::prelude::*;
use anchor_spl::{
    associated_token::AssociatedToken,
    token::{Mint, Token, TokenAccount, Transfer},
};
use jupiter_dca::cpi::{self};
use crate::constants::PDA_SEED;

#[derive(Accounts)]
#[instruction(application_idx: u64)]
pub struct SetupDca<'info> {
    /// CHECK: Jup DCA will check
    jup_dca: UncheckedAccount<'info>,

    /// CHECK: Jup DCA will check
    jup_dca_pda: UncheckedAccount<'info>,

    /// CHECK: Jup DCA will check
    jup_dca_in_ata: UncheckedAccount<'info>,

    /// CHECK: Jup DCA will check
    jup_dca_out_ata: UncheckedAccount<'info>,

    /// CHECK: Jup DCA will check
    jup_dca_event_authority: UncheckedAccount<'info>,

    input_mint: Account<'info, Mint>,
    output_mint: Account<'info, Mint>,

    #[account(mut)]
    user: Signer<'info>,

    #[account(
        mut,
        token::authority=user,
        token::mint=input_mint,
    )]
    user_token_account: Account<'info, TokenAccount>,

    #[account(
      init,
      payer = user,
      space = Pda::LEN,
      seeds = [PDA_SEED, user.key().as_ref(), input_mint.key().as_ref(), output_mint.key().as_ref(), application_idx.to_le_bytes().as_ref()],
      bump
    )]
    pda: Account<'info, Pda>,

    #[account(
      init,
      payer=user,
      associated_token::authority=pda,
      associated_token::mint=input_mint,
    )]
    pda_in_ata: Account<'info, TokenAccount>,

    #[account(
      init,
      payer=user,
      associated_token::authority=pda,
      associated_token::mint=output_mint,
    )]
    pda_out_ata: Account<'info, TokenAccount>,

    system_program: Program<'info, System>,
    token_program: Program<'info, Token>,
    associated_token_program: Program<'info, AssociatedToken>,
}

pub fn setup_dca(
    ctx: Context<SetupDca>,
    application_idx: u64,
    in_amount: u64,
    in_amount_per_cycle: u64,
    cycle_frequency: i64,
    min_out_amount: Option<u64>,
    max_out_amount: Option<u64>,
    start_at: Option<i64>,
    close_wsol_in_ata: Option<bool>,
) -> Result<()> {
    anchor_spl::token::transfer(
        CpiContext::new(
            ctx.accounts.token_program.to_account_info(),
            Transfer {
                from: ctx.accounts.user_token_account.to_account_info(),
                to: ctx.accounts.pda_in_ata.to_account_info(),
                authority: ctx.accounts.user.to_account_info(),
            },
        ),
        in_amount,
    )?;

    let open_dca_accounts = cpi::accounts::OpenDca {
        input_mint: ctx.accounts.input_mint.to_account_info(),
        output_mint: ctx.accounts.output_mint.to_account_info(),
        dca: ctx.accounts.jup_dca_pda.to_account_info(),
        user: ctx.accounts.pda.to_account_info(),
        user_ata: ctx.accounts.pda_in_ata.to_account_info(),
        in_ata: ctx.accounts.jup_dca_in_ata.to_account_info(),
        out_ata: ctx.accounts.jup_dca_out_ata.to_account_info(),
        event_authority: ctx.accounts.jup_dca_event_authority.to_account_info(),
        program: ctx.accounts.jup_dca.to_account_info(),
        system_program: ctx.accounts.system_program.to_account_info(),
        token_program: ctx.accounts.token_program.to_account_info(),
        associated_token_program: ctx.accounts.associated_token_program.to_account_info(),
    };

    let idx_bytes = ctx.accounts.pda.idx.to_le_bytes();
    let signer_seeds: &[&[&[u8]]] = &[pda_seeds!(ctx.accounts.pda, idx_bytes)];

    let cpi_ctx = CpiContext::new_with_signer(
        ctx.accounts.jup_dca.to_account_info(),
        open_dca_accounts,
        signer_seeds,
    );

    cpi::open_dca(
        cpi_ctx,
        application_idx,
        in_amount,
        in_amount_per_cycle,
        cycle_frequency,
        min_out_amount,
        max_out_amount,
        start_at,
        close_wsol_in_ata,
    )?;

    Ok(())
}