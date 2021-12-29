use anchor_lang::prelude::*;

declare_id!("69s61JXjofTGqs5GwpPm41WY2128MCKi2GzPeLRrExUh");

#[program]
pub mod myepicproject {
  use super::*;

  pub fn start_stuff_off(ctx: Context<StartStuffOff>) -> ProgramResult {
    // Get a reference to the account.
    let base_account = &mut ctx.accounts.base_account;
    // Initialize total_gifs.
    base_account.total_gifs = 0;
    Ok(())
  }

  pub fn add_gif(ctx: Context<UserStruct>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;

    let item = ItemStruct {
      gif_link: gif_link.to_string(),
      user_address: *user.to_account_info().key,
      upvotes: Vec::new(),
      total_upvotes: 0
    };

    let iter = &mut base_account.gif_list.iter();

    if iter.any(|x| x.gif_link == gif_link) {
      panic!("This gif has already been added!");
    }
		else{
	// Add it to the gif_list vector.
    base_account.gif_list.push(item);
    base_account.total_gifs += 1;
    Ok(())
    }
  }

  pub fn upvote(ctx: Context<UserStruct>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let user_address = *user.to_account_info().key;

    let item = base_account.gif_list.iter_mut().find(|x| x.gif_link == gif_link);
    
    let item_ref = &mut item.unwrap();
    let votes_iter = &mut item_ref.upvotes.iter();
    if votes_iter.any(|&y| y == user_address) {
      panic!("You already upvoted this item!")
    }
    else{
      item_ref.upvotes.push(user_address);
      item_ref.total_upvotes += 1;
      Ok(())  
    }
  }

  pub fn remove_upvote(ctx: Context<UserStruct>, gif_link: String) -> ProgramResult {
    let base_account = &mut ctx.accounts.base_account;
    let user = &mut ctx.accounts.user;
    let user_address = *user.to_account_info().key;

    let item = base_account.gif_list.iter_mut().find(|x| x.gif_link == gif_link);
    
    let item_ref = &mut item.unwrap();
    let votes_iter = &mut item_ref.upvotes.iter();

    let index = votes_iter.position(|y| *y == user_address);

    match index {
      Some(v) => println!("{:?}", v),
      None => panic!("You haven't upvoted this item!"),
    }

    let index_val = index.unwrap();
    item_ref.upvotes.swap_remove(index_val);
    item_ref.total_upvotes -= 1;
    Ok(())  
  }
}

#[derive(Accounts)]
pub struct StartStuffOff<'info> {
    #[account(init, payer = user, space = 9000)]
    pub base_account: Account<'info, BaseAccount>,
    #[account(mut)]
    pub user: Signer<'info>,
    pub system_program: Program <'info, System>,
}

#[derive(Accounts)]
pub struct UserStruct<'info> {
  #[account(mut)]
  pub base_account: Account<'info, BaseAccount>,
  #[account(mut)]
  pub user: Signer<'info>,
}

// Create a custom struct for us to work with.
#[derive(Debug, Clone, AnchorSerialize, AnchorDeserialize)]
pub struct ItemStruct {
    pub gif_link: String,
    pub user_address: Pubkey,
    pub upvotes: Vec<Pubkey>,
    pub total_upvotes: u64,
}

// Tell Solana what we want to store on this account.
#[account]
pub struct BaseAccount {
    pub total_gifs: u64,
    pub gif_list: Vec<ItemStruct>,
}