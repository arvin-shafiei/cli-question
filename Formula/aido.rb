class Aido < Formula
  desc "AI-powered command generation and execution for your terminal"
  homepage "https://github.com/arvin-shafiei/cli-question"
  url "https://github.com/arvin-shafiei/cli-question/archive/refs/tags/v0.1.0.tar.gz"
  sha256 "199768d46e166324296908dc8f1e9de9ad0fc2c8f93a53d08f6ccc02bf5f5496"
  license "MIT"
  head "https://github.com/arvin-shafiei/cli-question.git", branch: "main"

  depends_on "rust" => :build

  def install
    cd "aido" do
      system "cargo", "install", *std_cargo_args
    end
  end

  test do
    assert_match "AI-powered command generation", shell_output("#{bin}/aido --help")
  end
end
