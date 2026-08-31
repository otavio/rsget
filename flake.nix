{
  inputs = {
    garnix-lib.url = "github:garnix-io/garnix-lib";
  };

  nixConfig = {
    extra-substituters = [ "https://cache.garnix.io" ];
    extra-trusted-public-keys = [ "cache.garnix.io:CTFPyKSLcx5RMJKfLo5EEPUObbA78b0YQ2DTCJXqr9g=" ];
  };

  outputs = inputs: inputs.garnix-lib.lib.mkModules {
    modules = [
    ];

    config = { pkgs, ... }: {

      garnix.deployBranch = "master";
    };
  };
}
