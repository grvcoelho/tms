class Tms < Formula
    desc "A CLI tool for managing tmux sessions with git projects"
    homepage "https://github.com/grvcoelho/tms"
    url "https://github.com/grvcoelho/tms/archive/refs/tags/v1.1.1.tar.gz"
    sha256 "replace_with_actual_sha256"
    license "MIT"
  
    depends_on "go" => :build
  
    def install
      system "go", "build", *std_go_args(ldflags: "-s -w")
    end
  
    test do
      assert_match "tms version 1.1.1", shell_output("#{bin}/tms --version")
    end
end
