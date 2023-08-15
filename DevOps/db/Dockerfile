FROM python

RUN pip install flask requests redis

COPY . /src/

WORKDIR /src/

ENV REDIS_HOST=redis

CMD ["python", "Script.py"]