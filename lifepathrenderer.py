HEADER = """\\documentclass[12pt]{report}
\\usepackage{geometry}
\\usepackage{array}
\\usepackage{tabularx}
\\usepackage{multicol}
\\usepackage{color, colortbl}
\\usepackage{changepage}
\\definecolor{Gray}{gray}{0.9}\\begin{document}
\\newgeometry{margin=1cm}
\\noindent
"""
START_STRING = """\\begin{{minipage}}{{\\columnwidth}}
\\begin{{tabularx}}{{\\textwidth}}{{>{{\\raggedright}}p{{3.5cm}}p{{0.85cm}}p{{0.85cm}}p{{0.85cm}}X}}
\\rowcolor{{Gray}} \\textbf{{{name}}} & {years}yrs & {res} & {stats} & \\textit{{{leads}}}
\\end{{tabularx}} \\vspace*{{-5pt}}"""
SKILL_STRING = """\\begin{{adjustwidth}}{{1cm}}{{}}\\textbf{{
\\textit{{Skills:}}}} {skillpoints}pts: {skills}\\end{{adjustwidth}}"""
GENERAL_SKILL_STRING = """\\begin{{adjustwidth}}{{1cm}}{{}}\\textbf{{
\\textit{{Skills:}}}} {generalpoints}pts: General\\end{{adjustwidth}}"""
BOTH_SKILL_STRING = """\\begin{{adjustwidth}}{{1cm}}{{}}\\textbf{{
\\textit{{Skills:}}}} {skillpoints}pts: {skills}; {generalpoints}pts: General\\end{{adjustwidth}}"""
TRAIT_STRING = """\\begin{{adjustwidth}}{{1cm}}{{}}\\textbf{{
\\textit{{Traits:}}}} {traitpoints}pts: {traits}\\end{{adjustwidth}}"""
REQS_STRING = """\\begin{{adjustwidth}}{{1cm}}{{}}\\textbf{{
\\textit{{Requirements:}}}} {requirements}\\end{{adjustwidth}}"""
SPECIAL_STRING = """\\begin{{adjustwidth}}{{1cm}}{{}}\\textbf{{
\\textit{{Special:}}}} {special}\\end{{adjustwidth}}"""
END_STRING = """\\vspace*{10pt} \\end{minipage}"""
FOOTER = """
\\end{document}
"""

import sys
import re
import os


def render_lifepath(lifepath, indices):
    stats = lifepath[indices['stats']]
    if stats == '':
        stats = '-'
    result = START_STRING.format(name=lifepath[indices['name']],
                                 years=lifepath[indices['years']],
                                 res=lifepath[indices['res']],
                                 stats=stats,
                                 leads=lifepath[indices['leads']])
    if lifepath[indices['general_skill_points']] != '0' \
            and lifepath[indices['skill_points']] != '0':
        result += BOTH_SKILL_STRING.format(
            skillpoints=lifepath[indices['skill_points']],
            skills=lifepath[indices['skills']],
            generalpoints=lifepath[indices['general_skill_points']])
    elif lifepath[indices['general_skill_points']] != '0':
        result += GENERAL_SKILL_STRING.format(
            generalpoints=lifepath[indices['general_skill_points']])
    else:
        result += SKILL_STRING.format(
            skillpoints=lifepath[indices['skill_points']],
            skills=lifepath[indices['skills']])
    traits = lifepath[indices['traits']]
    if traits == '':
        traits = '-'
    result += TRAIT_STRING.format(
        traitpoints=lifepath[indices['trait_points']],
        traits=traits)
    result += END_STRING
    return result


dir_path = os.path.dirname(os.path.realpath(__file__))

print(dir_path)

if len(sys.argv) < 2:
    print("No input file provided.")
    sys.exit(0)
input_file = sys.argv[1]

content = str()
with open(input_file, "r+") as f:
    content = f.read()
    content = content.split('\n')
    for id, c in enumerate(content):
        elements = re.split(''',(?=(?:[^"]|'[^']*'|"[^"]*")*$)''', c)
        elements = [x.strip(' "') for x in elements]
        content[id] = elements

headers = content[0]
content = content[1:]
indices = dict()
for idx, e in enumerate(headers):
    indices[e.strip()] = idx

content = sorted(content, key=lambda c: int(c[indices['id']]))

current_setting = 'literally no setting lul'

result_string = HEADER

for c in content:
    if c[indices['use']] != 'TRUE':
        continue
    if c[indices['setting']].strip(' ') != current_setting:
        current_setting = c[indices['setting']].strip(' ')
        result_string += "\\section*{{{setting}" \
            .format(setting=current_setting)
        if c[indices['born']].strip(' ') != 'TRUE':
            result_string += ' Subsetting}'
        else:
            result_string += ' Setting}'
    result_string += render_lifepath(c, indices)

result_string += FOOTER

with open('test.tex', 'w+') as f:
    f.write(result_string)

print(result_string)
