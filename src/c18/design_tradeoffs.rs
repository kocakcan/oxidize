/// Context: You are designing an application with a global configuration, e.g. containing
/// command-line flags.
///
/// Functionality: The application needs to pass immutable references to this configuration
/// throughout the application.
use std::rc::Rc;
use std::sync::Arc;

struct Config {
    flags: Flags,
    // .. more fields ..
}

// Option 1: use a reference
struct ConfigRef<'a>(&'a Config);

// Option 2: use a reference-counted pointer
struct ConfigRef(Rc<Config>);

// Option 3: use an atomic reference-counted pointer
struct ConfigRef(Arc<Config>);
/// In formal terms, this means that ConfigRef implements Send and Sync. Assuming Config: Send +
/// Sync, then both &Config and Arc<Config> satisfy this requirement, but Rc does not (because
/// non-atomic reference-counted pointers are not thread-safe). So Option 2 does not satisfy the
/// requirement, while Option 3 does.
///
/// We might also be tempted to conclude that Option 1 does not satisfy the requirement because
/// functions like thread::spawn require that all data moved into a thread can only contain
/// references with a 'static lifetime. However, that does not rule out Option 1 for two reasons:
///
///     1. The Config could be stored as a global static variable (e.g., using OnceLock), so one
///        could construct &'static Config references.
///     2. Not all concurrency mechanisms require 'static lifetimes, such as thread::scope.
/// Therefore, the requirement as-stated only rules out non-Send types, and we consider Option 1 and
/// 3 to be the correct answers.
/// Context: You are designing a simple asset manager for a game engine.
///
/// Functionality: An API client will provide paths of assets to load, and gets back access to the
/// loaded assets.
///
/// Assumptions: You may assume that loads happen synchronously and instantaneously.
///
/// Requirement: Once loaded, an asset should be permanently immutable.
pub struct AssetManager {
    // ...
}

// Option 1: return a reference
impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> &Asset;
}

// Option 2: return a mutable reference
impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> &mut Asset;
}

// Option 3: return a reference-counted pointer
impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> Rc<Asset>;
}

// Option 4: return a copyable handle that can be turned into a reference
#[derive(Copy, Clone)]
pub struct AssetHandle(usize);

impl AssetManager {
    pub fn load(&mut self, path: PathBuf) -> AssetHandle;
    pub fn get(&self, handle: AssetHandle) -> &Asset;
}
/// Context: The immutability of assets is enforced by types that don't permit mutating operations.
/// Both an immutable borrow &Asset and an immutable smart-pointer like Rc don't permit mutating
/// operations. Therefore, Options 1, 3, and 4 DO satisfy this requirement. Option 2 returns a
/// mutable borrow, which does NOT satisfy the requirement.
///
/// Context: 
