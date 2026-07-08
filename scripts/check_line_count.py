from pathlib import Path

SKIP = {'.git', 'target', 'node_modules'}
LIMIT = 600

blocked = []
for path in Path('.').rglob('*'):
    if not path.is_file():
        continue
    if SKIP & set(path.parts):
        continue
    try:
        lines = path.read_text(encoding='utf-8').splitlines()
    except UnicodeDecodeError:
        continue
    if len(lines) > LIMIT:
        blocked.append((str(path), len(lines)))

if blocked:
    for name, count in blocked:
        print(f'{count:5d} {name}')
    raise SystemExit('Files exceed 600-line ceiling')

print('All tracked text files are within the 600-line ceiling')
