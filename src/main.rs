use futures::stream::TryStreamExt;
use misskey::ClientExt;

#[tokio::main(flavor = "current_thread")]
async fn main() {
    // env_logger::init(); 

    let endpoint = std::env::var("MISSKEY_API_URL").expect("environment variable MISSKEY_API_URL");
    let token = std::env::var("MISSKEY_TOKEN").expect("environment variable MISSKEY_TOKEN");
    let client = misskey::HttpClient::builder(endpoint.as_str())
        .token(token)
        .build()
        .expect("Building misskey::HttpClient should succeed");


    let mut suspend_count_success = 0;
    let mut suspend_count_failures = 0;
    let mut delete_note_success = 0;
    let mut delete_note_failures = 0;

    for user_id in std::env::args().skip(1) {
        eprintln!("Doing {user_id}");
        let user: misskey::model::id::Id<_> = user_id.parse().expect("valid user_id");
        match client.suspend(user).await {
            Ok(()) => suspend_count_success += 1,
            Err(e) => {
                eprintln!("Failed to suspend user {user_id}: {e}");
                suspend_count_failures += 1;
            }
        }

        let mut notes = client.user_notes(user, ..);
        loop {
            match notes.try_next().await {
                Ok(None) => break,
                Ok(Some(note)) => match client.delete_note(&note).await {
                    Ok(()) => delete_note_success += 1,
                    Err(err) => {
                        eprintln!("Failed to delete note {}: {err}", note.id);
                        delete_note_failures += 1;
                    }
                },
                Err(err) => {
                    eprintln!("Failed to get user {user_id} notes: {err}");
                    delete_note_failures += 1;
                }
            }
        }
    }
    eprintln!("users suspended: {suspend_count_success}, not suspended: {suspend_count_failures}");
    eprintln!("notes deleted: {delete_note_success}, not deleted: {delete_note_failures}");
}
