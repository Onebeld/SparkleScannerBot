use std::env;
use sqlite3::{State, Statement};

/// Represents a link associated with a user.
pub struct Links {
    pub user_id: f64,
    pub link: String,
}

/// Adds a new link to the database for a given user.
///
/// # Arguments
///
/// * `user_id` - The ID of the user.
/// * `link` - The link to be added.
///
/// # Returns
///
/// The state of the database after adding the link.
///
/// # Panics
///
/// This function panics if the `DATABASE_URL` environment variable is not set or if there is a failure connecting to the database.
pub fn add_link(user_id: u64, link: &str) -> State {
    // Adding a new row to the database
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let connection = sqlite3::open(database_url).expect("Failed to connect to the database");

    let mut db = connection.prepare("INSERT INTO links VALUES (?, ?)").unwrap();

    // The numbers 1 and 2 denote the location of the question mark in the query
    db.bind(1, user_id.to_string().as_str()).unwrap();
    db.bind(2, link).unwrap();

    // Save the changes to the database
    db.next().unwrap()
}

/// Checks if a link exists for a given user.
///
/// # Arguments
///
/// * `user_id` - The ID of the user.
/// * `link` - The link to check.
///
/// # Returns
///
/// Returns `true` if the link exists for the user, `false` otherwise.
pub fn is_link_exists(user_id: u64, link: &str) -> bool {
    // We get the link list and check if there are any items in it
    let vec: Vec<Links> = get_all_links_from_user(user_id, Option::from(link));
    return vec.iter().count() > 0
}

/// Returns a vector of links for a given user ID and optional link.
///
/// If the `link` parameter is `Some`, the function will return all links
/// for the specified user ID that match the given link. If `link` is `None`,
/// it will return all links for the specified user ID.
///
/// # Arguments
///
/// * `user_id` - The ID of the user.
/// * `link` - An optional link to filter the results.
///
/// # Returns
///
/// A vector containing the links that match the specified user ID and link.
///
/// # Panics
///
/// This function will panic if the `DATABASE_URL` environment variable is not set
/// or if there is a problem connecting to the database.
pub fn get_all_links_from_user(user_id: u64, link: Option<&str>) -> Vec<Links> {
    let query: &str;

    // Depending on whether the reference is None, type in your query
    if link.is_some() {
        query = "SELECT * FROM links WHERE user_id = ? AND link = ?";
    }
    else {
        query = "SELECT * FROM links WHERE user_id = ?";
    }

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let connection = sqlite3::open(database_url).expect("Failed to connect to the database");

    let mut db = connection.prepare(query).unwrap();
    db.bind(1, user_id.to_string().as_str()).unwrap();

    // If there is a reference, bind the second value
    if let Some(str) = link {
        db.bind(2, str).unwrap();
    }

    // List
    let mut vec: Vec<Links> = Vec::new();

    // Get the rows and add a new link to the list
    add_to_vec_from_database(db, &mut vec);

    vec
}

/// Get all links from the database.
///
/// # Returns
///
/// A vector containing all the links found in the database.
///
/// # Panics
///
/// This function will panic if the `DATABASE_URL` environment variable is not set
/// or if there is a problem connecting to the database.
pub fn get_all_links() -> Vec<Links> {
    let query = "SELECT * FROM links";

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let connection = sqlite3::open(database_url).expect("Failed to connect to the database");

    let db = connection.prepare(query).unwrap();

    let mut vec: Vec<Links> = Vec::new();

    add_to_vec_from_database(db, &mut vec);

    vec
}

/// Adds data from a database statement to a vector of Links.
///
/// # Arguments
///
/// * `db` - The database statement to retrieve data from.
/// * `vec` - The vector of Links to add the data to.
///
/// # Example
///
/// ```
/// use std::vec::Vec;
///
/// struct Links {
///     user_id: f64,
///     link: String,
/// }
///
/// let mut db: Statement = ...; // database statement
/// let mut links_vec: Vec<Links> = Vec::new();
///
/// // Add data from the database statement to the vector
/// add_to_vec_from_database(db, &mut links_vec);
/// ```
fn add_to_vec_from_database(mut db: Statement, vec: &mut Vec<Links>) {
    while let State::Row = db.next().unwrap() {
        vec.push(Links {
            user_id: db.read::<f64>(0).unwrap(),
            link: db.read::<String>(1).unwrap(),
        })
    }
}

/// Clears all links associated with a user.
///
/// # Arguments
///
/// * `user_id` - The ID of the user whose links should be cleared.
///
/// # Returns
///
/// The state after clearing the links.
///
/// # Panics
///
/// This function will panic if the `DATABASE_URL` environment variable is not set
/// or if there is a problem connecting to the database.
pub fn clear_all_links(user_id: u64) -> State {
    // Specify in the request that we want to delete all histories in which the user ID matches the required one
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let connection = sqlite3::open(database_url).expect("Failed to connect to the database");
    let mut db = connection.prepare("DELETE FROM links WHERE user_id = ?").unwrap();

    db.bind(1, user_id.to_string().as_str()).unwrap();
    
    // Also, don't forget to save the changes
    db.next().unwrap()
}

/// Deletes some links from the database for a given user ID.
///
/// # Arguments
///
/// * `user_id` - The ID of the user.
/// * `links` - A vector of links to be deleted.
///
/// # Panics
///
/// This function panics if the `DATABASE_URL` environment variable is not set or if there is a failure
/// to connect to the database.
///
/// # Examples
///
/// ```rust
/// let user_id = 123;
/// let links = vec!["http://example.com", "http://example.org"];
///
/// delete_some_links(user_id, links);
/// ```
pub fn delete_some_links(user_id: u64, links: Vec<&str>) {
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL is not set");
    let connection = sqlite3::open(database_url).expect("Failed to connect to the database");

    for link in links {
        let mut db = connection.prepare("DELETE FROM links WHERE user_id = ? AND link = ?").unwrap();

        db.bind(1, user_id.to_string().as_str()).unwrap();
        db.bind(2, link).unwrap();

        db.next().unwrap();
    }
}

#[cfg(test)]
mod database_test {
    use super::*;

    #[test]
    fn test_insert_into_database() {
        add_link(654352, "Hello world!");
        add_link(654352, "No");
        add_link(654352, "Yes");
        add_link(654352, "Ggg");
        add_link(3552, "Lol");
        add_link(3552, "Go");

        assert!(true)
    }

    #[test]
    fn test_is_link_exists() {
        let bool1 = is_link_exists(654352, "Ggg");
        let bool2 = is_link_exists(654352, "Gg");

        println!("Is exist: {}", bool1);
        println!("Is exist: {}", bool2);

        assert!(true)
    }

    #[test]
    fn test_get_histories() {
        let vec = get_all_links_from_user(654352, None);

        for one_link in vec {
            println!("User ID: {} | Link: {}", one_link.user_id, one_link.link);
        }

        assert!(true)
    }

    #[test]
    fn test_clear_all_links() {
        clear_all_links(654352);
        clear_all_links(3552);

        println!("Histories for {} is cleared!", 654352f64);
        println!("Histories for {} is cleared!", 3552f64);

        assert!(true)
    }
}