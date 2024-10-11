class Tms < Formula
  desc "A CLI tool for managing tmux sessions with git projects"
  homepage "https://github.com/grvcoelho/tms"
  url "https://github.com/grvcoelho/tms/archive/refs/tags/v1.3.0.tar.gz"
  sha256 "772fe190483870878541702da969c76405ad0ab5df41b7f169d7521bf9f94f8d"
  license "Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tms version 1.3.0", shell_output("#{bin}/tms --version")
  end
end
