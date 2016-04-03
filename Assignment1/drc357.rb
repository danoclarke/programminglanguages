#!/usr/bin/ruby

################  METHODS  ################

##  MATH  ##
def add(a,b)
	res = a.to_i + b.to_i
	return res.to_s
end

def sub(a,b)
	res = a.to_i - b.to_i
	return res.to_s
end

def mult(a,b)
	res = a.to_i * b.to_i
	return res.to_s
end

def compute(arr)
	len = arr.length

	##Remove any blank entries
	while arr.index("") != nil
		pos = arr.index("")
		arr.delete_at(pos)
	end

	##Check array for potential negative ints
	if arr.include?("-")
		if arr.index("-") == 0
			arr[0] = "-" + arr[1]
			arr.delete_at(1)
		end

		for i in 1..len
			if arr[i] == "-" and ["+","-","*"].include?(arr[i-1])
				arr[i] = "-" + arr[i+1]
				arr.delete_at(i+1)
			end
		end			
	end

	##Loop and compute
	while arr.index("*") != nil
		pos = arr.index("*")
		arr[pos] = mult(arr[pos-1],arr[pos+1])
		arr.delete_at(pos+1)
		arr.delete_at(pos-1)
	end

	while arr.index("+") != nil
		pos = arr.index("+")
		arr[pos] = add(arr[pos-1],arr[pos+1])
		arr.delete_at(pos+1)
		arr.delete_at(pos-1)
	end

	while arr.index("-") != nil
		pos = arr.index("-")
		arr[pos] = sub(arr[pos-1],arr[pos+1])
		arr.delete_at(pos+1)
		arr.delete_at(pos-1)
	end

	##Return final value or false (error)
	if arr.length == 1
		return arr[0]
	else
		return false
	end
end

##  SYNTAX CHECK  ##
def isexpr(a)
	flag = false

	if a.include? "+" and not flag
		i = 0
		while a.index("+",i) != nil do
			plus = a.index("+",i)
			first = a[0...plus]
			last = a[plus+1..-1]
			if isexpr(first) and isterm(last)
				flag = true
				break
			end
			i = a.index("+",i) + 1
		end
	end

	if a.include? "-" and not flag 
		i = 0
		while a.index("-",i) != nil do
			plus = a.index("-",i)
			first = a[0...plus]
			last = a[plus+1..-1]
			if isexpr(first) and isterm(last)
				flag = true
				break
			end
			i = a.index("-",i) + 1
		end
	end

	if isterm(a) and not flag 
		flag = true
	end

	return flag
end

def isterm(a)
	tflag = false

	if a.include? "*" and not tflag
		i = 0
		while a.index("*",i) != nil do
			plus = a.index("*",i)
			first = a[0...plus]
			last = a[plus+1..-1]
			if isterm(first) and ispexpr(last)
				tflag = true
			end
			i = a.index("*",i) + 1
		end
	end

	if ispexpr(a) and not tflag 
		tflag = true
	end

	return tflag
end

def ispexpr(a)
	pflag = false

	if a[0] == "(" and a[-1] == ")" and not pflag
		a[0] = ''
		a[-1] = ''
		if isexpr(a)
			pflag = true
		end
	end

	if a[0]=="-" and not pflag
		a[0] = ''
		pflag = ispexpr(a)
	end

	if isint(a) and not pflag
		pflag = true
	end

	return pflag
end

def isint(a)
	return a.to_i.to_s == a
end

def checkinput(line)
	if isexpr(line)
		return true
	else
		return false
	end
end

########################  MAIN  ########################
input = Array.new
output = Array.new
error = ""

input = $stdin.readlines

##puts input
len = input.length

if input[len-1] == "QUIT\n" or input[len-1] == "quit\n"
	input.delete_at(len-1) ##remove quit and newline

	##CHECK INPUT SYNTAX##
	for line in input
		##Check that last char of line ends with a newline
		if line[-1] != "\n"
			error = "ERR1"
			break
		end

		##Check for digits seperated solely by whitespace.
		pattern = /\d+\s+\d+/
		if pattern.match(line) != nil
			error = "ERR2"
		end

		line = line.gsub(/\s+/,"").gsub(/\n+/,"").gsub(/\r+/,"") ##remove whitespace, newlines, returns
		
		unless checkinput(line)
			error = "ERR3"
			break
		end
	end

	if error == ""
		for line in input
			line = line.gsub(/\s+/,"").gsub(/\n+/,"").gsub(/\r+/,"") ##remove whitespace, newlines, returns
			larr = line.gsub("(","( ").gsub(")"," )").gsub("+"," + ").gsub("-"," - ").gsub("*"," * ") ##seperate ints from symbols
			larr = larr.split(/\s+/) ##split line
			
			stacks = Array.new
			stacks[0] = Array.new
			i = 0
			
			for char in larr
				if char == "("
					i += 1
					stacks[i] = Array.new
				elsif not ["+","-","*",")"].include?(char) ##number
					stacks[i].push(char)
				elsif ["+","-","*"].include?(char)
					stacks[i].push(char)
				elsif char == ")"
					res = compute(stacks[i])
					
					if res == false
						error = "ERR4"
					end

					stacks[i].clear ## Clear current stack
					i -= 1
					stacks[i].push(res) ## Push result to previous stack
				else
					error = "ERR5"
					break
				end
			end

			if stacks[0].length > 1
				ans = compute(stacks[0])
			else
				ans = stacks[0][0] ##if whole eq was enclosed in ( )
			end

			if error == ""
				puts ans
			else
				puts error
			end
		end
	else
		puts error
	end
else
	error = "ERR6"
	puts error
end