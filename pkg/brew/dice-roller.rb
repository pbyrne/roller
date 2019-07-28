class DiceRoller < Formula
  version "1.0.0"
  desc "Roll dice from the command line."
  homepage "https://github.com/pbyrne/roller"

  if OS.mac?
      url "https://github.com/pbyrne/roller/releases/download/v#{version}/dice-roller-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "7af757c4cc9c41023031ebc0d67245b80de46cc6e07ef0d7a0368438c0cd2173"
  elsif OS.linux?
      url "https://github.com/pbyrne/roller/releases/download/v#{version}/dice-roller-v#{version}-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "7df56571413a77ec1ff4998c7d504d086530b5140afd54d51bb86f1df4f9033c"
  end

  def install
    bin.install "roll"
  end
end
