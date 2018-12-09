#!/usr/bin/env python3
import requests

for prefix in ['', 'with-msg/', 'public-err/']:
    for error_case in ['internal', '404']:
        for a in ['a', 'b']:
            url = 'http://localhost:8000/' + prefix + error_case + '/a/' + a
            print('[*] ' + url)
            r = requests.get(url)
            status = r.status_code
            json = r.json()

            if a == 'a':
                if status != 200:
                    raise Exception('Wrong status code')
                body = json['success']
            else:
                if error_case == 'internal' and status != 500:
                    raise Exception('Wrong status code')
                if error_case == '404' and status != 404:
                    raise Exception('Wrong status code')
                body = json['error']

                if prefix == 'with-msg/' and body != 'hello':
                    raise Exception('Wrong response body')
                if prefix == 'public-err/' and body != 'this error is ok to leak':
                    raise Exception('Wrong response body')
