require 'os'

target = "dotnetdetect.exe"
tp = "target\\release\\dotnetdetect.exe"

if !OS.windows? then
    puts "This program should only be built for Windows."
    exit 1
end

task :default do
    sh "cargo build --release"
end

task :upx => [:default] do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "upx -9 #{tp} -o #{target}"
end

task :clean do
    if File.exists?(target) then
        File.delete(target)
    end
    sh "cargo clean"
end

task :cleanlock do 
    if File.exists?("Cargo.lock") then
        File.delete("Cargo.lock")
    end
end
