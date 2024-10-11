class Tms < Formula
  desc "A CLI tool for managing tmux sessions with git projects"
  homepage "https://github.com/grvcoelho/tms"
  url "https://github.com/grvcoelho/tms/archive/refs/tags/v1.3.3.tar.gz"
  sha256 "560cda6d83c502443043b457866a4a805d3dc17569bc1c02bcb5b8fefdf37020"
  license "Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tms version 1.3.3", shell_output("#{bin}/tms --version")
  end
end
