use std::{fmt::Display, fs, io::{self, stdin, stdout}, path::PathBuf};
use argon2::Argon2;
use chacha20poly1305::{
    aead::{Aead, AeadCore, KeyInit, OsRng}, ChaCha20Poly1305, Key, Nonce
};


#[derive(Debug)]
struct Cfg {
    storage: PathBuf
}

impl Default for Cfg {
    fn default() -> Self {
        let home = dirs::home_dir()
            .unwrap();
        let storage: PathBuf = [ 
            home,
            PathBuf::from(".secman")
        ]
        .iter()
        .collect();

        Cfg {
            storage
        }
    }
}

#[derive(Debug)]
enum InitErrorKind {
    CreateStorageDir(io::Error)
}

#[derive(Debug)]
enum Error {
    SkeletonKey(argon2::Error),
    Init(InitErrorKind)
}


impl Display for Error {
    fn fmt(
        &self,
        f: &mut std::fmt::Formatter<'_>
    ) -> std::fmt::Result {
        write!(f, "{:#?}", self)
    }
}

impl std::error::Error for Error { }

impl From<io::Error> for Error {
    fn from(value: io::Error) -> Self {
        Error::Init(InitErrorKind::CreateStorageDir(value))
    }
}

fn init_firstrun(
    cfg: &Cfg
) -> Result<(), Error> {
    fs::create_dir(&cfg.storage)?;

    Ok(())
}

fn is_first_run(cfg: &Cfg) -> bool {
    !cfg.storage.exists()
}

fn init(cfg: &Cfg) -> Result<(), Error> {
    if is_first_run(cfg) {
        init_firstrun(cfg)?;
    }

    Ok(())
}

enum Command {
    Ls,
    Add,
    Rm
}

struct Entry {
    name: String,
    nonce: String,
    path: PathBuf
}

fn entries(
    cfg: &Cfg
) -> Result<(), std::io::Error> {
    let mut files = fs::read_dir(&cfg.storage)?
        .map(|res| res.map(|e| e.path()))
        .collect::<Result<Vec<_>, io::Error>>()?;
    files.sort();

    let entries: Vec<Entry> = files
        .into_iter()
        .map(|f| Entry { name: todo!(), nonce: todo!(), path: todo!() })
        .collect();

    Ok(())
}

fn main() -> Result<(),  Box<dyn std::error::Error>> {
    
    let cfg = Cfg::default();

    let (stdin, stdout) = (stdin(), stdout());
    
    let mut user_input = String::with_capacity(64);
    stdin.read_line(&mut user_input)?;

    let mut output_key_material = [0u8; 32];
    let password = user_input.as_bytes();

    let mut salt = Vec::from(b"example salt");

    if let Err(e) = Argon2::default().hash_password_into(
        password, 
        &salt, 
        &mut output_key_material
    ) {
        return Err(Box::new(Error::SkeletonKey(e)))
    }

    let key = Key::from(output_key_material);
    let cipher = ChaCha20Poly1305::new(&key);
    let nonce = ChaCha20Poly1305::generate_nonce(&mut OsRng); // 96-bits; unique per message

    /* let ciphertext = cipher.encrypt(&nonce, b"plaintext message".as_ref())?;
    let plaintext = cipher.decrypt(&nonce, ciphertext.as_ref())?; */

    user_input.clear();
    
    salt.fill(0);

    dbg!(cfg);
    
    Ok(())
}
