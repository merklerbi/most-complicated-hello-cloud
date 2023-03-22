! curl.f90
program main
    use, intrinsic :: iso_fortran_env, only: stderr => error_unit
    implicit none
    character(len=*), parameter :: URL    = 'http://worldtimeapi.org/api/timezone/Europe/Berlin.txt'
    character(len=*), parameter :: PARENT = '/tmp/'

    character(len=512)            :: buf
    character(len=:), allocatable :: cmd, tmp_file
    integer                       :: file_unit, rc

    call random_seed()

    ! Set output of cURL to random file, for example `/tmp/aUqCmPev.tmp`.
    call random_file(file_path=tmp_file, parent=PARENT, n=12, ext='.tmp')
    cmd = 'curl -s -o "' // tmp_file // '" "' // URL // '"'

    ! Run cURL from command-line.
    call execute_command_line(cmd, exitstat=rc)

    if (rc /= 0) then
        write (stderr, '(a, i0)') 'Error: HTTP request failed: ', rc
        stop
    end if

    ! Open the temporary file.
    open (action='readwrite', file=tmp_file, iostat=rc, newunit=file_unit, status='old')

    if (rc /= 0) then
        write (stderr, '(3a, i0)') 'Error: reading file "', tmp_file, '" failed: ', rc
        stop
    end if

    ! Read and output contents of file.
    do
        read (file_unit, '(a)', iostat=rc) buf
        if (rc /= 0) exit
        print '(a)', trim(buf)
    end do

    ! Close and delete file.
    close (file_unit, status='delete')
contains
    subroutine random_file(file_path, parent, n, ext)
        !! Returns a random file path in string `file_path`, with parent
        !! directory 'parent`, for instance: `/tmp/aUqCmPev.tmp`.
        !!
        !! The returned file name contains `n` random characters in range
        !! [A-Za-z], plus the given extension.
        character(len=:), allocatable, intent(inout) :: file_path
        character(len=*),              intent(in)    :: parent
        integer,                       intent(in)    :: n
        character(len=*),              intent(in)    :: ext

        character(len=n) :: tmp
        integer          :: i, l
        real             :: r(n)

        file_path = parent
        l = len(parent)
        if (parent(l:l) /= '/') file_path = file_path // '/'

        call random_number(r)

        do i = 1, n
            if (r(i) < 0.5) then
                tmp(i:i) = achar(65 + int(25 * r(i)))
            else
                tmp(i:i) = achar(97 + int(25 * r(i)))
            end if
        end do

        file_path = file_path // tmp // ext
    end subroutine random_file
end program main