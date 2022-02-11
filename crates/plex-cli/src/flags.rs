xflags::xflags! {
    src "./src/flags.rs"

    cmd plex-cli {
        /// Server URL, e.g. http://192.168.1.1:32400. Default is http://127.0.0.1:32400.
        optional -s, --server url: String

        /// Authentication token, if needed. Mandatory for the claimed server.
        optional -t, --token token: String

        default cmd help {
            /// Print help information.
            optional -h, --help
        }

        /// Wait for the server to be available.
        cmd wait {
            /// Delay between attempts
            optional -d, --delay seconds: u32

            /// How long to wait for the success.
            optional --timeout seconds: u32
        }

        /// Manage server preferences.
        cmd preferences {
            /// Get current configuration.
            cmd get {
                /// Return only one parameter.
                optional -k, --key key: String

                /// Return all parameters from the specified group.
                optional -g, --group group: String

                /// Display advanced configuration.
                optional --show-advances

                /// Display hidden parameters.
                optional --show-hidden
            }

            /// Modify the configuration.
            cmd set {
                /// Parameter name.
                required -k, --key key: String

                /// New value.
                required -v, --value value: String

                /// Required if you're changing a hidden parameter.
                optional --i-know-what-i-am-doing
            }
        }
    }
}

// generated start
// The following code is generated by `xflags` macro.
// Run `env UPDATE_XFLAGS=1 cargo build` to regenerate.
#[derive(Debug)]
pub struct PlexCli {
    pub server: Option<String>,
    pub token: Option<String>,
    pub subcommand: PlexCliCmd,
}

#[derive(Debug)]
pub enum PlexCliCmd {
    Help(Help),
    Wait(Wait),
    Preferences(Preferences),
}

#[derive(Debug)]
pub struct Help {
    pub help: bool,
}

#[derive(Debug)]
pub struct Wait {
    pub delay: Option<u32>,
    pub timeout: Option<u32>,
}

#[derive(Debug)]
pub struct Preferences {
    pub subcommand: PreferencesCmd,
}

#[derive(Debug)]
pub enum PreferencesCmd {
    Get(Get),
    Set(Set),
}

#[derive(Debug)]
pub struct Get {
    pub key: Option<String>,
    pub group: Option<String>,
    pub show_advances: bool,
    pub show_hidden: bool,
}

#[derive(Debug)]
pub struct Set {
    pub key: String,
    pub value: String,
    pub i_know_what_i_am_doing: bool,
}

impl PlexCli {
    pub const HELP: &'static str = Self::HELP_;

    #[allow(dead_code)]
    pub fn from_env() -> xflags::Result<Self> {
        Self::from_env_()
    }

    #[allow(dead_code)]
    pub fn from_vec(args: Vec<std::ffi::OsString>) -> xflags::Result<Self> {
        Self::from_vec_(args)
    }
}
// generated end

impl Help {
    pub(crate) fn run(self) -> anyhow::Result<()> {
        println!("{}", PlexCli::HELP);
        Ok(())
    }
}

impl PlexCli {
    pub async fn run(self) -> anyhow::Result<()> {
        let server = self
            .server
            .clone()
            .unwrap_or_else(|| "http://127.0.0.1:32400".to_owned());

        let auth_token = self.token.unwrap_or_default();

        match self.subcommand {
            PlexCliCmd::Help(cmd) => cmd.run(),
            PlexCliCmd::Wait(cmd) => cmd.run(&server, &auth_token).await,
            PlexCliCmd::Preferences(cmd) => cmd.run(&server, &auth_token).await,
        }
    }
}
