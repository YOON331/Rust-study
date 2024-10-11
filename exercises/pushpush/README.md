# PushPush  
- [Push Push 게임](https://flasharch.com/en/archive/play/ea901d336b660426ef35e2ccf4170666)
- 이 프로그램은 PushPush 게임을 커맨드라인 인터페이스에서 실행할 수 있는 기능을 제공합니다.   
- 이 PushPush는 2개의 스테이지로 구분되어 화면의 `*`을 `0`위치로 이동시켜서 모든 0을 `@`로 만들면 게임이 종료됩니다.   <br/><br/>
- 실행 화면 


## Features
**게임 화면을 출력하는 함수 `display_map`**   
  - Vec\<Vec\<i32>>의 map을 화면에 출력합니다.   
  <br/>   

**map을 업데이트 하는 함수 `set_map`**   
  - 현재 상태들을 바탕으로 map의 정보를 업데이트합니다.   
  <br/>   

**눈사람을 움직일 수 있는지 확인하는 함수 `move_op`**   
  - 입력받은 커맨드라인에 따라 눈사람(8)을 움직일 수 있는지 확인합니다.   
  <br/>   

**게임 종료 상태를 확인하는 함수 `chk_complete`**   
  - 완료 상태(@)를 충족하는지 확인합니다


## 실행하기
**1. git clone**
  ~~~
  git clone https://github.com/YOON331/Rust-study.git
  ~~~
<br/>

**2. 폴더 이동 및 실행**
  - 폴더 이동
    ~~~
    cd exercises/pushpush
    ~~~
  - 실행 
    ~~~
    cargo run  -- stage <stage num>
    ~~~
    - stage 1: easy, stage 2: normal
<br/>

**3. 커맨드라인 입력하면서 게임 즐기기**  
- 요소 설명   
`8`: 이동 가능한 눈사람   
`*`: 이동 가능한 별  
`0`: 비어있는 창고     
`@`: 창고(0) 안에 별(*)이 들어간 완료 상태   
`1`: 움직일 수 있는 길   <br/><br/>

- 커맨드라인 설명   
`q`: 게임 종료   
`r`: 게임 초기화  
`w`: 위쪽으로 1칸 이동   
`s`: 아래쪽으로 1칸 이동      
`a`: 왼쪽으로 1칸 이동   
`d`: 오른쪽으로 1칸 이동   
