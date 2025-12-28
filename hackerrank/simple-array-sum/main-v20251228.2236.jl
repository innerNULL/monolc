#
# Complete the 'simpleArraySum' function below.
#
# The function is expected to return an INTEGER.
# The function accepts INTEGER_ARRAY ar as parameter.
#

function simpleArraySum(ar::Array{Int32})::Int32
    out::Int32 = 0;
    for x in ar
        out += x;
    end
    out;
end

fptr = open(ENV["OUTPUT_PATH"], "w")

ar_count = parse(Int32, strip(readline(stdin)))

ar = map(x -> parse(Int32, x), Array{String}(split(rstrip(readline(stdin)))))

result = simpleArraySum(ar)

write(fptr, string(result) * "\n")

close(fptr)

