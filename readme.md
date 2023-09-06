# prime number calc by rust lang

러스트로 만든 소수 구하기 프로그램 입니다.

multi thread와 single thread 버전이 있습니다.

go 언어로 만들었던 https://github.com/kasworld/primenum 의 rust 버전 입니다.

발견한 소수들을 primes.data.\<elementtypename\> 에 저장하고 실행할때 읽으니  
성능 테스트를 원하면 primes.data.\<elementtypename\> 를 지운후 실행하세요. 

    프로그램 수자 명령 
    형태로 실행하고 
    수자 : 작은쪽으로 찻거나, 까지 계산
    명령 : 
        single : single thread로 계산  
        find : multi thread 로 검색 
        calc : multi thread 로 계산 