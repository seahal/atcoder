str = gets.chomp
hash = Hash.new(0)
str.scan(/[a-z]/).each {|v|
  hash[v] += 1
}
puts hash.sort{|a,b| [b[1], a[0]] <=> [a[1], b[0]] }.first.first
