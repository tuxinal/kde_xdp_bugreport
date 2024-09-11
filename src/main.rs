use anyhow::Result;
use ashpd::{desktop::global_shortcuts, WindowIdentifier};
use futures::StreamExt;

#[tokio::main]
async fn main() -> Result<()> {
    // initialize global shortcuts
    let gs_instance = global_shortcuts::GlobalShortcuts::new().await?;
    let session = gs_instance.create_session().await?;
    
    // should return something after the first run (bug: doesn't)
    let shortcuts = gs_instance.list_shortcuts(&session).await?.response()?;
    println!("{:?}", shortcuts);

    // register shortcuts if there are no shorcuts registered (bug: can't find registered shortcuts. always registers)
    if shortcuts.shortcuts().len() == 0 {
        let shortcut1 = global_shortcuts::NewShortcut::new("xdp_testing1", "quit program!")
            .preferred_trigger("ALT+n");
        let shortcut2 = global_shortcuts::NewShortcut::new("xdp_testing2", "does something!")
            .preferred_trigger("ALT+s");
        gs_instance
            .bind_shortcuts(&session, &[shortcut1, shortcut2], &WindowIdentifier::None)
            .await?;
    }

    let mut stream = gs_instance.receive_activated().await?;
    while let Some(shortcut) = stream.next().await {
        match shortcut.shortcut_id() {
            "xdp_testing1" => {
                println!("closing!");
                session.close().await?;
                break;
            },
            "xdp_testing2" => {
                println!("did something!");
            },
            _ => {}
        }
    }
    Ok(())
}
