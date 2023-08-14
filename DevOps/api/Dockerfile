FROM python:latest

RUN pip install flask requests redis

COPY ./server.py /src/

EXPOSE 5000

CMD ["python3", "/src/server.py"]