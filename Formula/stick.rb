class Stick < Formula
  desc "Minimal static site generator in Rust"
  homepage "https://github.com/dask-58/stick"
  version "0.1.0"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/dask-58/stick/releases/download/v0.1.0/stick-aarch64-apple-darwin.tar.gz"
      sha256 "3d2aa09dd59036f1dcfb6d8f0f22d60b18f585c77910b7295e3ba07b10178e55"
    end
    on_intel do
      url "https://github.com/dask-58/stick/releases/download/v0.1.0/stick-x86_64-apple-darwin.tar.gz"
      sha256 "3d2aa09dd59036f1dcfb6d8f0f22d60b18f585c77910b7295e3ba07b10178e55"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/dask-58/stick/releases/download/v0.1.0/stick-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "3d2aa09dd59036f1dcfb6d8f0f22d60b18f585c77910b7295e3ba07b10178e55"
    end
    on_intel do
      url "https://github.com/dask-58/stick/releases/download/v0.1.0/stick-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "3d2aa09dd59036f1dcfb6d8f0f22d60b18f585c77910b7295e3ba07b10178e55"
    end
  end

  def install
    bin.install "stick"
  end

  test do
    system "#{bin}/stick", "--version"
  end
end
