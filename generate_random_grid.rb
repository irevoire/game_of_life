if ARGV.size != 2
  puts "Usage:
    provide the width and the height of the grid you want to generate"
    exit -1
end

width = ARGV[0].to_i
height = ARGV[1].to_i

possible_value = [' ', '#']

(0...height).each do
  (0..width).each do
    print possible_value.sample + ","
  end
  puts possible_value.sample
end

