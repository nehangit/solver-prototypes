from prance import ResolvingParser
parser = ResolvingParser('openapi.yml', backend = 'openapi-spec-validator')
print(parser.json())