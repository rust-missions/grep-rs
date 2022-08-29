# grep-rs : 멀티 쓰레드 그렙

- Rust 기반 grep 프로그램
- 복수의 파일이 선택된 경우, 쓰레드풀을 생성하여 개별 파일에 대한 검색 작업을 각 쓰레드에서 동시에 처리

## 명령어 설치 방법 (MacOs)

프로그램 다운로드 및 퍼미션 변경

```bash
curl -o /usr/local/bin/grep-rs 'https://raw.githubusercontent.com/bugoverdose/grep-rs/main/target/debug/grep-rs'
chmod 755 /usr/local/bin/grep-rs
```

실행방법

```bash
grep-rs search-keyword relative-path
```

프로그램 제거

```bash
rm -rf /usr/local/bin/grep-rs
```

## 실행 결과 예시

해당 프로젝트의 [example](./example) 디렉토리 내부에서 grep-rs 명령어를 실행한 경우 아래와 같이 출력됩니다.

#### 단일 파일에 대한 실행 결과

```bash
$ grep-rs somebody poem
poem 5 - How dreary to be somebody!
```

#### 디렉토리에 대한 실행 결과

```bash
$ grep-rs somebody inner_directory
inner_directory/poem1.txt 5 - How dreary to be somebody!
inner_directory/poem2.txt 2 - This is a little story about four people named everybody, somebody, anybody, and nobody.
inner_directory/poem2.txt 3 - There was an important job to be done and everybody was sure that somebody would do it.
inner_directory/poem2.txt 5 - somebody got angry about that because it was everybody's job.
inner_directory/poem2.txt 7 - It ended up that everybody blamed somebody when nobody did what anybody could have done
```

#### 내부에 디렉토리가 포함된 디렉토리에 대한 실행 결과

```bash
$ grep-rs somebody .
./inner_directory/poem1.txt 5 - How dreary to be somebody!
./inner_directory/poem2.txt 2 - This is a little story about four people named everybody, somebody, anybody, and nobody.
./inner_directory/poem2.txt 3 - There was an important job to be done and everybody was sure that somebody would do it.
./inner_directory/poem2.txt 5 - somebody got angry about that because it was everybody's job.
./inner_directory/poem2.txt 7 - It ended up that everybody blamed somebody when nobody did what anybody could have done
Failed to search inside './image.png'
./poem 5 - How dreary to be somebody!
```

#### 검색 대상이 텍스트 파일이 아닌 경우

```bash
$ grep-rs somebody image.png
Failed to search inside 'image.png'
```

#### 대소문자 구분

```bash
$ grep-rs Somebody inner_directory/poem2.txt
inner_directory/poem2.txt 0 - Everybody, Somebody, Anybody, and Nobody
```

```bash
$ grep-rs somebody inner_directory/poem2.txt
inner_directory/poem2.txt 2 - This is a little story about four people named everybody, somebody, anybody, and nobody.
inner_directory/poem2.txt 3 - There was an important job to be done and everybody was sure that somebody would do it.
inner_directory/poem2.txt 5 - somebody got angry about that because it was everybody's job.
inner_directory/poem2.txt 7 - It ended up that everybody blamed somebody when nobody did what anybody could have done
```

## 기능 및 동작원리

- 프로그램 실행 시점에 `grep-rs keyword path` 형식으로 두 개의 인자를 전달 받는다.
    - keyword : 개별 파일에서 찾을 검색어
    - path : 명령어를 실행한 위치로부터의 상대 경로. 파일, 디렉토리 모두 허용.

- path에 해당되는 파일이 존재하는 경우, 해당 파일에서 keyword를 포함하는 줄을 찾아 `path line_idx - line_content` 형식으로 출력한다.
    - line_idx : 해당 파일 내에서 몇 번째 줄인가에 대한 정보. 0부터 시작.
    - line_content : 해당 줄의 내용.

- path에 해당되는 디렉토리가 존재하는 경우, 디렉토리 내 모든 파일에 대해 검사를 진행한다.
    - 복수의 쓰레드에서 개별 파일을 맡아 동시에 작업을 수행한다.
    - 쓰레드풀을 구현하여 최대 10개의 쓰레드만을 생성하여 재사용한다.
    - 동시에 수행해야 하는 작업이 적은 경우, 불필요한 쓰레드는 생성하지 않는다.
    - 해당 디렉토리 내부에 또 다른 디렉토리가 존재하는 경우, 위의 작업을 반복한다.

- 검색 대상 파일이 하나인 경우, 쓰레드풀을 생성하지 않고 즉시 검색 작업을 수행한다.

## 예외처리

- 검색 대상에 텍스트로 구성되지 않은 파일이 포함된 경우, 프로그램을 종료하지 않고 해당 파일에 대한 검색에 실패했음을 출력한다.
- 프로그램 실행시 전달되는 인자의 개수가 두 개보다 적거나 많은 경우, 예외 메시지를 출력하며 프로그램을 종료한다.
- path에 해당되는 파일 및 디렉토리가 존재하지 않는 경우, 예외 메시지를 출력하며 프로그램을 종료한다.
- path가 가리키는 디렉토리 내부에 파일이 없는 경우, 예외 메시지를 출력하며 프로그램을 종료한다.
