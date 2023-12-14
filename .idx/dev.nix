{pkgs}: {
  # Which nixpkgs channel to use.
  channel = "stable-23.05"; # or "unstable"

  # Use https://search.nixos.org/packages to  find packages
  packages = [
    pkgs.gccgo13
    pkgs.rustup
  ];

  # sets environment variables in the workspace
  # env = {
  #   SOME_ENV_VAR = "hello";
  # };

  # search for the extension on https://open-vsx.org/ and use "publisher.id"
  idx.extensions = [
    "rust-lang.rust-analyzer"
  ];
}