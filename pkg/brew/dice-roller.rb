class DiceRoller < Formula
  version "1.0.0"
  desc "Roll dice from the command line."
  homepage "https://github.com/pbyrne/roller"

  if OS.mac?
      url "https://github.com/pbyrne/roller/releases/download/v#{version}/dice-roller-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "ac026d00f6f16765e70b25cd73122157ef72f5364ae552ec0864b8b34896814b"
  elsif OS.linux?
      url "https://github.com/pbyrne/roller/releases/download/v#{version}/dice-roller-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "3af76865d3877ee67a958e6e4aa517efda93dbeba7b29d55e865fd96d3b47968"
  end

  def install
    bin.install "roll"
  end
end
