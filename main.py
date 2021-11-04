class LispAtom:
    def __init__(self, val):
        self.val = val
    def eval(self, env):
        return self.val

class LispExpr:
    """Either a list of LispExpr or a LispExpr or a LispAtom"""
    def __init__(self, expr):
        self.expr = expr
    def eval(self, env):
        return LispAtom(self.expr[0].eval(env)(
            *map(lambda e: e.eval(env), self.expr[1:])))

class LispEnv:
    def __init__(self, parent=None):
        self.parent = parent
        self.table = {} # dict of (string, LispAtom)
    
    def get(self, name):
        if name in self.table.keys():
            return self.table[name]
        if not self.parent:
            return None
        return self.parent.get(name)
    
    def add(self, name, expr):
        self.table[name] = expr

class LispName:
    """Instances of this are variables"""
    def __init__(self, name):
        self.name = name
    def eval(self, env):
        return env.get(self.name).eval(env)

class LispFunc:
    """This should be a LispName"""
    def __init__(self, args: list, body: LispExpr):
        self.args = args
        self.body = body

    def eval(self, env: LispEnv, params: dict):
        # this is the tricky part. we create a new child environment here
        local_env = LispEnv(parent=env)
        for a, p in params.items():
            local_env.add(a, p)
        return self.body.eval(local_env)

env_global = LispEnv()

env_global.add('+', LispAtom(lambda x, y: x + y))
env_global.add('-', LispAtom(lambda x, y: x - y))
env_global.add('*', LispAtom(lambda x, y: x * y))


expr = LispExpr([LispAtom(lambda x, y: x + y), LispName("x"), LispName("y")])

print(expr.eval(env_global).eval(env_global))
