use crate::{error::RmError, search, utils, Config, NodeModuleMap};

// Run the program
pub fn run(config: Config) -> Result<NodeModuleMap, RmError> {
    // Check target_dir
    utils::is_directory_valid(&config.target_dir)?;

    // @TODO: Spinner start here
    // // Create spinner & begin search in separate threads
    // let spinner_handle = spinner::init_spinner(is_searching);
    match search::init_search(&config) {
        // @TODO: SPinner end here
        // @TODO: return results from search
        Ok(res) => {
            // @TODO: remove returned directories & return total removed size

            Ok(res)
        }
        Err(e) => Err(e),
    }
}
