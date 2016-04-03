from sys import stdin

def get_type(a):
    if is_primitive_type(a):
        return 'Primitive'
    elif is_typevar(a):
        return 'Variable'
    elif is_listtype(a):
        return 'List'
    elif is_functype(a):
        return 'Function'

def get_vars(a):
    if is_primitive_type(a):
        return a
    elif is_typevar(a):
        return a
    elif is_listtype(a):
        return '[' + get_vars(a[1:-1]) + ']'
    elif is_functype(a):
        ret = []
        parts = a.split('->',1)
        parts[0] = parts[0].strip()
        parts[1] = parts[1].strip()
        types = parts[0][1:-1].split(',')
        for e in types:
            ret.append(e.strip())
        ret.append(get_vars(parts[1]))
        return ret

def check_input(line):
##    print(line)
    types = line.split('^')
    if len(types) == 2:
        return (is_type(types[0]) and is_type(types[1]))
    else:
        return False

def is_type(a):
##    print("is-type ",a)
    return (is_typevar(a) or is_primitive_type(a) or is_functype(a) or is_listtype(a))

def is_primitive_type(a):
##    print("is-ptype ",a)
    if type(a) is not str:
        return False
    
    a = a.strip()

    if a == 'int' or a == 'real' or a == 'str':
        return True
    else:
        return False

def is_typevar(a):
##    print("is-typevar ",a)
    a = a.strip()

    return (a[0] == '`' and is_varname(a[1:]))

def is_varname(a):
##    print("is-varname ",a)

    if len(a) > 1:
        return (a[0].isalpha() and a[1:].isalnum())
    else:
        return a.isalpha()

def is_functype(a):
##    print("is-functype ",a)
    parts = a.split('->',1)

    if len(parts) == 2:
        list = parts[0].strip()

        if list[0] == '(' and list[-1] == ')':
            if list[1:-2] != '':
                return (is_arglist(list[1:-1]) and is_type(parts[1]))
            else:
                return is_type(parts[1])
    else:
        return False

def is_arglist(a):
##    print("is-arglist ",a)
    a = a.strip()

    splitter = a.split(',')

    flag = False

    for e in splitter:
        flag = is_type(e.strip())

    return flag

def is_listtype(a):
##    print("is-listtype ",a)
    a = a.strip()

    return (a[0] == '[' and a[-1] == ']' and is_type(a[1:-1].strip()))

def unify(a,b):
    global ptrs
    
    if get_type(a) == get_type(b): ##same type
        if is_listtype(a):
            contents1 = a[1:-1]
            contents2 = b[1:-1]
            if is_typevar(contents1):
                if contents1 not in ptrs.keys():
                    ptrs[contents1] = contents2
            elif is_typevar(contents2):
                if contents2 not in ptrs.keys():
                    ptrs[contents2] = contents1
            else:
                print('BOTTOM') ## must be primitive (but they are not identical)
                return False
        elif is_primitive_type(a): ##but not exact same
            print(a,b)
            print('BOTTOM')
            return False
        else: ## variable type, point one to other
            if a not in ptrs.keys():
                ptrs[a] = b
            elif b not in ptrs.keys():
                ptrs[b] = a
            elif ptrs[a] != b and ptrs[b] != a:
                print('BOTTOM')
                return False
    else: ##different type
        ## list/prim
        if (is_listtype(a) and is_primitive_type(b)) or (is_primitive_type(a) and is_listtype(b)):
            print('BOTTOM')
            return False
        ## list/var
        elif (is_listtype(a) and is_typevar(b)) or (is_typevar(a) and is_listtype(b)):
            if is_listtype(a):
                if b not in ptrs.keys():
                    ptrs[b] = a
            else:
                if a not in ptrs.keys():
                    ptrs[a] = b
        ## prim/var
        elif (is_primitive_type(a) and is_typevar(b)) or (is_typevar(a) and is_primitive_type(b)):
            if is_primitive_type(a):
                if b not in ptrs.keys():
                    ptrs[b] = a
                else:
                    if ptrs[b] != a:
                        print('BOTTOM')
                        return False
            else:
                if a not in ptrs.keys():
                    ptrs[a] = b
                else:
                    if ptrs[a] != b:
                        print('BOTTOM')
                        return False

    return True

def unify_lists(type1_vars,type2_vars):
    for i in range(0,len(type1_vars)):
        if type(type1_vars[i]) is list: ##return is a function
            unify_lists(type1_vars[i],type2_vars[i])
        else:
            if type1_vars[i] != type2_vars[i]:
                if unify(type1_vars[i],type2_vars[i]):
                    update(type1_vars)
                    update(type2_vars)
                else:
                    return False
    return True

def update(a):
    global ptrs

    if type(a) == list:
        for i in range(0,len(a)):
            if type(a[i]) is list:
                update(a[i])
            elif is_listtype(a[i]):
                contents = a[i][1:-1].strip()
                if contents in ptrs.keys():
                    a[i] = '[' + ptrs[contents] + ']'
            elif a[i] in ptrs.keys():
                if is_listtype(ptrs[a[i]]):
                    ## see if contents of list is in ptrs, and update that
                    contents = ptrs[a[i]][1:-1].strip()
                    if contents in ptrs.keys():
                        ptrs[a[i]] = '[' + ptrs[contents] + ']'
                a[i] = ptrs[a[i]]
    else:
        if a in ptrs.keys():
            if is_listtype(ptrs[a]):
                ## see if contents of list is in ptrs, and update that
                contents = ptrs[a][1:-1].strip()
                if contents in ptrs.keys():
                    ptrs[a] = '[' + ptrs[contents] + ']'
            return ptrs[a]

def print_functype(type1_vars):
    print_string = "("
    for e in type1_vars[:-1]:
        print_string += e + ","
    print_string = print_string[:-1] ## strip last ','
    print_string += ") -> "
    if type(type1_vars[-1]) is list:
        print_string += print_functype(type1_vars[-1])
    else:
        print_string += type1_vars[-1]

    return print_string
    
ptrs = {}

while(True):
    line = stdin.readline()

    if line == 'QUIT\n':
        break
    elif(check_input(line[:-1])):
        types = line[:-1].split('^')

        type1_vars = get_vars(types[0].strip())
        type2_vars = get_vars(types[1].strip())

        if type(type1_vars) is list and type(type2_vars) is list:
            if not unify_lists(type1_vars,type2_vars):
                break
        else:
            if type(type1_vars) == type(type2_vars):
                if type1_vars != type2_vars:
                    if unify(type1_vars,type2_vars):
                        type1_vars = update(type1_vars)
                        type2_vars = update(type2_vars)
                    else:
                        break
            else:
                print('ERR')
                break

        if type(type1_vars) is list:
            print(print_functype(type1_vars))
        else:
            print(type1_vars)
    else:
        print('ERR')
        break
            
