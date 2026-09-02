{
  rustPlatform,
  lib,
}:
rustPlatform.buildRustPackage (final: {
  pname = "tempy";
  version = "0.0.0";

  src = ./.;

  cargoHash = "sha256-64a0gb6hdVTc2znASRsmUX3zVd4Ff/1VVdUZO0mzjxU=";

  meta = {
    description = "Convert temperature values between fahrenheit and celsius.";
    homepage = "https://github.com/fuguesoft/tempy";
    license = lib.licenses.gpl3;
    maintainers = with lib.maintainers; [ fuguesoft ];
  };
})
