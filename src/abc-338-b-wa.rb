str = gets.chomp
hash = Hash.new(0)
str.scan(/./).each {|v|
  hash[v] += 1
}
puts hash.lazy.sort_by{ _1[1]}.last.first
