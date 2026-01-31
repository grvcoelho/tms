class Tms < Formula
  desc "A CLI tool for managing tmux sessions with git projects"
  homepage "https://github.com/grvcoelho/tms"
  url "https://github.com/grvcoelho/tms/archive/refs/tags/v1.4.1.tar.gz"
  sha256 "466c79c0e7009888dc812e7941ceadfcfbcaed60aaeaba0b306ebd7466ed67b5"
  license "Apache-2.0"

  depends_on "rust" => :build
  depends_on "ghq"
  depends_on "fzf"
  depends_on "tmux"

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tms version 1.4.1", shell_output("#{bin}/tms --version")
  end
end
