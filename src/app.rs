use crate::auth::TokenSet;
use crate::contacts::ContactService;
use crate::prompt::prompt_index;
use crate::session::AppSession;
use crate::tui::run_contacts_tui;
use anyhow::Result;

#[derive(Debug, Clone)]
pub struct AppMenuOptions {
    pub app_api_base_url: String,
    pub contacts_page: u32,
    pub contacts_per_page: u32,
}

pub fn run_menu(tokens: &TokenSet, session: &AppSession, options: AppMenuOptions) -> Result<()> {
    println!("\n{} - {}\n", session.account_name, session.app_description);

    loop {
        println!("{}", render_menu());
        match prompt_index("Choose menu item: ", 3)? {
            1 => println!("Overview is not implemented yet.\n"),
            2 => {
                let contacts = ContactService::new(&options.app_api_base_url)?.list_contacts(
                    tokens,
                    session,
                    options.contacts_page,
                    options.contacts_per_page,
                )?;
                if let Some(contact) = run_contacts_tui(contacts)? {
                    println!(
                        "Selected contact: {} <{}>\n",
                        contact.full_name, contact.email
                    );
                }
            }
            3 => return Ok(()),
            _ => unreachable!(),
        }
    }
}

pub fn render_menu() -> &'static str {
    "Menu:\n  1. Overview\n  2. Contacts\n  3. Quit"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_initial_menu_items() {
        let rendered = render_menu();

        assert!(rendered.contains("1. Overview"));
        assert!(rendered.contains("2. Contacts"));
        assert!(rendered.contains("3. Quit"));
    }
}
