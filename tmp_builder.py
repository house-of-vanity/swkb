from jinja2 import Environment, PackageLoader, FileSystemLoader

cargo = dict()


# async-std = { version = "1.6", features = ["attributes", "unstable"], optional = true }



def parse_line(val):
  result = dict()
  attr, value = line.split("=", maxsplit=1)
  result[value] = dict()

  return result

def parse(filename='Cargo.toml'):
  with open(filename) as cargo_file:
      level = ''
      for line in cargo_file.readlines():
        if line[0] == '#' or line[0] == '\n':
          continue

        print("line is ", (line, len(line)))

        if line[0] == '[':
          attr = line.replace('[', '').replace(']', '').strip()
          cargo[attr] = dict()
          level = attr
          continue

        value = parse_line(line)
        print(value)
        cargo[level] += value

print(cargo)

config = {}

with open("PKGBUILD", "w") as rcfile_obj:
    file_loader = Environment(loader=FileSystemLoader('assets'))
    template = file_loader.get_template('PKGBUILD.jinja')
    output = template.render(config=config)
    rcfile_obj.write(output+ "\n")
