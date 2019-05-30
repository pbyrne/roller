class DiceRoller < Formula
  version "0.1.3"
  desc "Roll dice from the command line."
  homepage "https://github.com/pbyrne/roller"

  if OS.mac?
      url "https://github.com/pbyrne/roller/releases/download/v#{version}/dice-roller-v#{version}-x86_64-apple-darwin.tar.gz"
      sha256 "92446c6b28b6c726f91ad66a03bcd533fc6e1a28ef4b44c27bfe2d49a0f88531"
  elsif OS.linux?
      url "https://github.com/pbyrne/roller/releases/download/v#{version}/dice-roller-v#{version}-x86_64-unknown-linux-musl.tar.gz"
      sha256 "ce74cabac9b39b1ad55837ec01e2c670fa7e965772ac2647b209e31ead19008c"
  end

  def install
    bin.install "roll"
  end
end
