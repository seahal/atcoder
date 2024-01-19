# 競技プログラミングの鉄則(ISBN: 978-4-8399-7750-4)
# A11 - Binary Search 1

nr, x = gets.split(" ").map(&:to_i)
a = gets.split(" ").map(&:to_i)

left = 0

while left <= nr do
  mean = (left + nr) / 2
  if a[mean] < x
    left = mean + 1
  elsif a[mean] == x
    puts mean + 1
    break;
  else
    nr = mean - 1
  end
end
