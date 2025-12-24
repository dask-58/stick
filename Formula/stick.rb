class Stick < Formula
  desc "Minimal static site generator in Rust"
  homepage "https://github.com/dask-58/stick"
  version "0.1.2"
  license "MIT"

  on_macos do
    on_arm do
      url "https://github.com/dask-58/stick/releases/download/v#{version}/stick-aarch64-apple-darwin.tar.gz"
    end
    on_intel do
      url "https://github.com/dask-58/stick/releases/download/v#{version}/stick-x86_64-apple-darwin.tar.gz"
    end
  end

  on_linux do
    on_arm do
      url "https://github.com/dask-58/stick/releases/download/v#{version}/stick-aarch64-unknown-linux-gnu.tar.gz"
    end
    on_intel do
      url "https://github.com/dask-58/stick/releases/download/v#{version}/stick-x86_64-unknown-linux-gnu.tar.gz"
    end
  end

  def install
    bin.install "stick"
  end

  test do
    system bin/"stick", "--version"
  end
end
