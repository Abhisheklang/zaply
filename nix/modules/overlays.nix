{ inputs, ... }:
{
  flake.overlays.default =
    final: _:
    let
      mkZaply = import ../toolchain.nix { inherit inputs; };
    in
    {
      zaply-editor = mkZaply final;
    };
}
