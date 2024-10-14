class Tms < Formula
  desc "A CLI tool for managing tmux sessions with git projects"
  homepage "https://github.com/grvcoelho/tms"
  url "https://github.com/grvcoelho/tms/archive/refs/tags/v1.4.0.tar.gz"
  sha256 "74858ddb3d3d56cd44ed3785b70985f7a95bffe9127fe30a23003168aa253434"
  license "Apache-2.0"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    assert_match "tms version 1.4.0", shell_output("#{bin}/tms --version")
  end
end
