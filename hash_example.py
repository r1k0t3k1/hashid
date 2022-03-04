from urllib import response
import requests
import bs4

f = open('hashes.rs', 'w',encoding='UTF-8')

f.write('''
struct hash {
  mode: usize,
  name: String,
  example: String,
}

fn new_hash_table() -> Vec<hash> {
  let hashes: Vec<hash> = vec!();  

''')

url = 'https://hashcat.net/wiki/doku.php?id=example_hashes'

response = requests.get(url)

html = bs4.BeautifulSoup(response.text, 'html.parser')
tr = html.select('tr')

for r in tr:
  raw_col0 = r.find('td', { 'class': 'col0' })
  raw_col1 = r.find('td', { 'class': 'col1' })
  raw_col2 = r.find('td', { 'class': 'col2' })
  if raw_col0 is not None and raw_col1 is not None and raw_col2 is not None:
    r0 = raw_col0.text.strip(' ').replace('\n', '')
    r1 = raw_col1.text.strip(' ').replace('\n', '')
    r2 = raw_col2.text.strip(' ').replace('\n', '')
    f.write('  hases.push(hash { mode:%s,name:"%s",example:"%s"});\n' % (r0,r1,r2))

f.write('}')
f.close()
